use glow::HasContext;
use glutin::{
    api::egl::Egl,
    config::{Api, Config, ConfigSurfaceTypes, GlConfig},
    context::{AsRawContext, ContextApi, ContextAttributesBuilder, RawContext, Version},
    display::{AsRawDisplay, DisplayApiPreference, GetDisplayExtensions, GetGlDisplay},
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::AsRawSurface,
};
use khronos_egl::Downcast;
use once_cell::sync::Lazy;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard, RwLock};

use std::{
    collections::HashMap, ffi, mem::ManuallyDrop, num::NonZero, os::raw, ptr, rc::Rc, sync::Arc,
    time::Duration,
};

fn parse_egl_version(version_str: &str) -> Option<(i32, i32)> {
    // Expects format: "EGL {major}.{minor}"
    let parts: Vec<&str> = version_str.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let version_parts: Vec<&str> = parts[1].split('.').collect();
    if version_parts.len() != 2 {
        return None;
    }
    let major = version_parts[0].parse().ok()?;
    let minor = version_parts[1].parse().ok()?;
    Some((major, minor))
}

#[derive(Debug)]
struct EglContext {
    context: Arc<glutin::api::egl::context::PossiblyCurrentContext>,
    version: (i32, i32),
    display: glutin::api::egl::display::Display,
    pbuffer: Arc<glutin::api::egl::surface::Surface<glutin::surface::PbufferSurface>>,
}

impl EglContext {
    fn make_current(&self) {
        let _ = self.context.make_current(&self.pbuffer);
    }

    fn unmake_current(&mut self) {
        // TODO is make_not_current_in_place() okay, or should we switch to make_not_current?
        let _ = self.context.make_not_current_in_place();
    }
}

struct EglContextLock<'a> {
    context: &'a Arc<glutin::api::egl::context::PossiblyCurrentContext>,
    display: glutin::api::egl::display::Display,
}

/// A guard containing a lock to an [`AdapterContext`], while the GL context is kept current.
pub struct AdapterContextLock<'a> {
    glow: MutexGuard<'a, ManuallyDrop<glow::Context>>,
    egl: Option<EglContextLock<'a>>,
}

impl<'a> std::ops::Deref for AdapterContextLock<'a> {
    type Target = glow::Context;

    fn deref(&self) -> &Self::Target {
        &self.glow
    }
}

impl<'a> Drop for AdapterContextLock<'a> {
    fn drop(&mut self) {
        if let Some(egl) = self.egl.take() {
            egl.context.make_current_surfaceless().unwrap();
        }
    }
}

/// A wrapper around a [`glow::Context`] and the required EGL context that uses locking to guarantee
/// exclusive access when shared with multiple threads.
pub struct AdapterContext {
    glow: Mutex<ManuallyDrop<glow::Context>>,
    egl: Option<glutin::api::egl::context::PossiblyCurrentContext>,
}

unsafe impl Sync for AdapterContext {}
unsafe impl Send for AdapterContext {}

impl AdapterContext {
    pub fn is_owned(&self) -> bool {
        self.egl.is_some()
    }

    /// Returns the EGLDisplay corresponding to the adapter context.
    ///
    /// Returns [`None`] if the adapter was externally created.
    pub fn raw_display(&self) -> Option<glutin::api::egl::display::Display> {
        let display = match self.egl {
            Some(ref egl) => Some(egl.display()),
            None => None,
        };
        display
    }

    /// Returns the EGL version the adapter context was created with.
    ///
    /// Returns [`None`] if the adapter was externally created.
    pub fn egl_version(&self) -> Option<(i32, i32)> {
        let version = match self.egl {
            Some(ref egl) => {
                let display = egl.display();
                let version_str = display.version_string();
                Some(parse_egl_version(&version_str).unwrap())
            }
            None => None,
        };
        version
    }

    pub fn raw_context(&self) -> Option<RawContext> {
        match self.egl {
            Some(ref egl) => Some(egl.raw_context()),
            None => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum SrgbFrameBufferKind {
    /// No support for SRGB surface
    None,
    /// Using EGL 1.5's support for colorspaces
    Core,
    /// Using EGL_KHR_gl_colorspace
    Khr,
}

#[derive(Debug)]
struct Inner {
    /// Note: the context contains a dummy pbuffer (1x1).
    /// Required for `eglMakeCurrent` on platforms that doesn't supports `EGL_KHR_surfaceless_context`.
    egl: EglContext,
    #[allow(unused)]
    version: (i32, i32),
    supports_native_window: bool,
    config: glutin::api::egl::config::Config,
    // #[cfg_attr(Emscripten, allow(dead_code))]
    // wl_display: Option<*mut raw::c_void>,
    #[cfg_attr(Emscripten, allow(dead_code))]
    force_gles_minor_version: wgt::Gles3MinorVersion,
    /// Method by which the framebuffer should support srgb
    srgb_kind: SrgbFrameBufferKind,
}

impl Inner {
    fn create(
        flags: wgt::InstanceFlags,
        display: glutin::api::egl::display::Display,
        force_gles_minor_version: wgt::Gles3MinorVersion,
    ) -> Result<Self, crate::InstanceError> {
        profiling::scope!("Create EGL Context");

        let display_version = display.version_string();
        let version = parse_egl_version(&display_version).expect("EGL version cannot be parsed");
        let display_extensions = display.extensions();

        log::info!("Display version: {:?}", display_version);
        log::info!("Display extensions: {:?}", display_extensions);

        let srgb_kind = if version >= (1, 5) {
            log::debug!("\tEGL surface: +srgb");
            SrgbFrameBufferKind::Core
        } else if display_extensions.contains("EGL_KHR_gl_colorspace") {
            log::debug!("\tEGL surface: +srgb khr");
            SrgbFrameBufferKind::Khr
        } else {
            log::warn!("\tEGL surface: -srgb");
            SrgbFrameBufferKind::None
        };

        let mut template = glutin::config::ConfigTemplateBuilder::new()
            .prefer_hardware_accelerated(Some(true))
            .with_surface_type(ConfigSurfaceTypes::WINDOW.union(ConfigSurfaceTypes::PBUFFER))
            .with_api(Api::GLES2);
        if srgb_kind != SrgbFrameBufferKind::None {
            template = template.with_alpha_size(8);
        }

        let config = unsafe {
            display
                .find_configs(template.build())
                .unwrap()
                .next()
                .ok_or(crate::InstanceError::new("No EGL configs found".to_owned()))?
        };

        let supports_native_window = config
            .config_surface_types()
            .contains(ConfigSurfaceTypes::WINDOW);

        let context_attributes = ContextAttributesBuilder::new().build(None);

        // Since glutin by default tries to create OpenGL core context, which may not be
        // present we should try gles.
        let fallback_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(None);

        // There are also some old devices that support neither modern OpenGL nor GLES.
        // To support these we can try and create a 2.1 context.
        let legacy_context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
            .build(None);

        // TODO port from gles handle robustness, opengl / opengles

        let not_current_gl_context = unsafe {
            display
                .create_context(&config, &context_attributes)
                .unwrap_or_else(|_| {
                    display
                        .create_context(&config, &fallback_context_attributes)
                        .unwrap_or_else(|_| {
                            display
                                .create_context(&config, &legacy_context_attributes)
                                .expect("failed to create context")
                        })
                })
        };

        // Create a dummy pbuffer surface
        let attrs =
            glutin::surface::SurfaceAttributesBuilder::<glutin::surface::PbufferSurface>::new()
                .build(
                    NonZero::new(1).unwrap(), // width
                    NonZero::new(1).unwrap(), // height
                );

        // Testing if context can be binded without surface
        // and creating dummy pbuffer surface if not.
        // TODO: gles check if it supports surfaceless
        let pbuffer = unsafe {
            display
                .create_pbuffer_surface(&config, &attrs)
                .expect("Cannot create pbuffer_surface")
        };

        // Make context current
        let context = not_current_gl_context.make_current(&pbuffer).unwrap();

        Ok(Self {
            egl: EglContext {
                display,
                context: Arc::new(context),
                pbuffer: Arc::new(pbuffer),
                version,
            },
            version: (3, 0), // Example version
            supports_native_window,
            config,
            force_gles_minor_version,
            srgb_kind,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum WindowKind {
    Wayland,
    X11,
    AngleX11,
    Unknown,
}

#[derive(Clone, Debug)]
struct WindowSystemInterface {
    kind: WindowKind,
}

pub struct Instance {
    wsi: WindowSystemInterface,
    flags: wgt::InstanceFlags,
    inner: Mutex<Inner>,
}

impl Instance {}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}

impl crate::Instance for Instance {
    type A = super::Api;

    unsafe fn init(desc: &crate::InstanceDescriptor) -> Result<Self, crate::InstanceError> {
        profiling::scope!("Init OpenGL (EGL) Backend");

        let display = unsafe {
            glutin::api::egl::display::Display::new(
                desc.display
                    .expect("cannot create glutin instance without raw display handle")
                    .as_raw(),
            )
            .expect("couldn't create glutin display")
        };

        // TODO: print client extensions
        // TODO: Port gles backend debug code - context attributes builder has with_debug()

        let inner = Inner::create(desc.flags, display, desc.gles_minor_version)?;

        Ok(Instance {
            wsi: WindowSystemInterface {
                kind: WindowKind::Wayland,
            },
            flags: desc.flags,
            inner: Mutex::new(inner),
        })
    }

    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<<Self::A as crate::Api>::Surface, crate::InstanceError> {
        todo!()
    }

    unsafe fn enumerate_adapters(
        &self,
        surface_hint: Option<&<Self::A as crate::Api>::Surface>,
    ) -> Vec<crate::ExposedAdapter<Self::A>> {
        todo!()
    }
}

#[derive(Debug)]
pub struct Swapchain {
    surface: glutin::api::egl::surface::Surface,
    // TODO: remove the wl_window as we are having the raw_window_handle(somewhere)
    wl_window: Option<*mut raw::c_void>,
    framebuffer: glow::Framebuffer,
    renderbuffer: glow::Renderbuffer,
    /// Extent because the window lies
    extent: wgt::Extent3d,
    format: wgt::TextureFormat,
    format_desc: super::TextureFormatDesc,
    #[allow(unused)]
    sample_type: wgt::TextureSampleType,
}
#[derive(Debug)]
pub struct Surface {
    egl: EglContext,
    // wsi: WindowSystemInterface,
    config: glutin::api::egl::config::Config,
    pub(super) presentable: bool,
    raw_window_handle: raw_window_handle::RawWindowHandle,
    swapchain: RwLock<Option<Swapchain>>,
    srgb_kind: SrgbFrameBufferKind,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

impl Surface {
    pub(super) unsafe fn present(
        &self,
        _suf_texture: super::Texture,
        context: &AdapterContext,
    ) -> Result<(), crate::SurfaceError> {
        todo!("present");
        let gl = unsafe { context.get_without_egl_lock() };
        let swapchain = self.swapchain.read();
        let sc = swapchain.as_ref().unwrap();

        self.egl
            .instance
            .make_current(
                self.egl.display,
                Some(sc.surface),
                Some(sc.surface),
                Some(self.egl.raw),
            )
            .map_err(|e| {
                log::error!("make_current(surface) failed: {}", e);
                crate::SurfaceError::Lost
            })?;

        unsafe { gl.disable(glow::SCISSOR_TEST) };
        unsafe { gl.color_mask(true, true, true, true) };

        unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None) };
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(sc.framebuffer)) };

        if !matches!(self.srgb_kind, SrgbFrameBufferKind::None) {
            // Disable sRGB conversions for `glBlitFramebuffer` as behavior does diverge between
            // drivers and formats otherwise and we want to ensure no sRGB conversions happen.
            unsafe { gl.disable(glow::FRAMEBUFFER_SRGB) };
        }

        // Note the Y-flipping here. GL's presentation is not flipped,
        // but main rendering is. Therefore, we Y-flip the output positions
        // in the shader, and also this blit.
        unsafe {
            gl.blit_framebuffer(
                0,
                sc.extent.height as i32,
                sc.extent.width as i32,
                0,
                0,
                0,
                sc.extent.width as i32,
                sc.extent.height as i32,
                glow::COLOR_BUFFER_BIT,
                glow::NEAREST,
            )
        };

        if !matches!(self.srgb_kind, SrgbFrameBufferKind::None) {
            unsafe { gl.enable(glow::FRAMEBUFFER_SRGB) };
        }

        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None) };

        self.egl
            .instance
            .swap_buffers(self.egl.display, sc.surface)
            .map_err(|e| {
                log::error!("swap_buffers failed: {}", e);
                crate::SurfaceError::Lost
                // TODO: should we unset the current context here?
            })?;
        self.egl
            .instance
            .make_current(self.egl.display, None, None, None)
            .map_err(|e| {
                log::error!("make_current(null) failed: {}", e);
                crate::SurfaceError::Lost
            })?;

        Ok(())
    }

    unsafe fn unconfigure_impl(
        &self,
        device: &super::Device,
    ) -> Option<(khronos_egl::Surface, Option<*mut raw::c_void>)> {
        let gl = &device.shared.context.lock();
        match self.swapchain.write().take() {
            Some(sc) => {
                unsafe { gl.delete_renderbuffer(sc.renderbuffer) };
                unsafe { gl.delete_framebuffer(sc.framebuffer) };
                Some((sc.surface, sc.wl_window))
            }
            None => None,
        }
    }

    pub fn supports_srgb(&self) -> bool {
        match self.srgb_kind {
            SrgbFrameBufferKind::None => false,
            _ => true,
        }
    }
}

impl crate::Surface for Surface {
    type A = super::Api;

    unsafe fn configure(
        &self,
        device: &super::Device,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        let (surface, wl_window) = match unsafe { self.unconfigure_impl(device) } {
            Some(pair) => pair,
            None => {
                let mut wl_window = None;
                let attributes_builder = glutin::surface::SurfaceAttributesBuilder::<
                    glutin::surface::WindowSurface,
                >::new();
                // We don't want any of the buffering done by the driver, because we
                // manage a swapchain on our side.
                // Some drivers just fail on surface creation seeing `EGL_SINGLE_BUFFER`.
                // if cfg!(any(target_os = "android", target_os = "macos"))
                //     || cfg!(windows)
                //     || self.wsi.kind == WindowKind::AngleX11
                // {
                //     khronos_egl::BACK_BUFFER
                // } else {
                //     khronos_egl::SINGLE_BUFFER
                // },
                attributes_builder.with_single_buffer(false);
                if config.format.is_srgb() {
                    match self.srgb_kind {
                        SrgbFrameBufferKind::None => attributes_builder.with_srgb(None),
                        _ => attributes_builder.with_srgb(Some(true)),
                    };
                }
                let attributes = attributes_builder.build(
                    self.raw_window_handle,
                    NonZero::new(config.extent.width)
                        .expect("trying to configure the surface with a negative or zero width"),
                    NonZero::new(config.extent.height)
                        .expect("trying to configure the surface with a negative or zero height"),
                );
                todo!("create a surface using the glutin display");

                // match raw_result {
                //     Ok(raw) => (raw, wl_window),
                //     Err(e) => {
                //         log::warn!("Error in create_window_surface: {:?}", e);
                //         return Err(crate::SurfaceError::Lost);
                //     }
                // }
            }
        };

        // if let Some(window) = wl_window {
        //     let library = &self.wsi.display_owner.as_ref().unwrap().library;
        //     let wl_egl_window_resize: libloading::Symbol<WlEglWindowResizeFun> =
        //         unsafe { library.get(b"wl_egl_window_resize\0") }.unwrap();
        //     unsafe {
        //         wl_egl_window_resize(
        //             window,
        //             config.extent.width as i32,
        //             config.extent.height as i32,
        //             0,
        //             0,
        //         )
        //     };
        // }

        let format_desc = device.shared.describe_texture_format(config.format);
        let gl = &device.shared.context.lock();
        let renderbuffer = unsafe { gl.create_renderbuffer() }.map_err(|error| {
            log::error!("Internal swapchain renderbuffer creation failed: {error}");
            crate::DeviceError::OutOfMemory
        })?;
        unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, Some(renderbuffer)) };
        unsafe {
            gl.renderbuffer_storage(
                glow::RENDERBUFFER,
                format_desc.internal,
                config.extent.width as _,
                config.extent.height as _,
            )
        };
        let framebuffer = unsafe { gl.create_framebuffer() }.map_err(|error| {
            log::error!("Internal swapchain framebuffer creation failed: {error}");
            crate::DeviceError::OutOfMemory
        })?;
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(framebuffer)) };
        unsafe {
            gl.framebuffer_renderbuffer(
                glow::READ_FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::RENDERBUFFER,
                Some(renderbuffer),
            )
        };
        unsafe { gl.bind_renderbuffer(glow::RENDERBUFFER, None) };
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None) };

        let mut swapchain = self.swapchain.write();
        *swapchain = Some(Swapchain {
            surface,
            wl_window,
            renderbuffer,
            framebuffer,
            extent: config.extent,
            format: config.format,
            format_desc,
            sample_type: wgt::TextureSampleType::Float { filterable: false },
        });

        Ok(())
    }

    unsafe fn unconfigure(&self, device: &super::Device) {
        // if let Some((surface, wl_window)) = unsafe { self.unconfigure_impl(device) } {
        //     self.egl
        //         .instance
        //         .destroy_surface(self.egl.display, surface)
        //         .unwrap();
        //     if let Some(window) = wl_window {
        //         let library = &self
        //             .wsi
        //             .display_owner
        //             .as_ref()
        //             .expect("unsupported window")
        //             .library;
        //         let wl_egl_window_destroy: libloading::Symbol<WlEglWindowDestroyFun> =
        //             unsafe { library.get(b"wl_egl_window_destroy\0") }.unwrap();
        //         unsafe { wl_egl_window_destroy(window) };
        //     }
        // }
    }

    unsafe fn acquire_texture(
        &self,
        _timeout_ms: Option<Duration>, //TODO
        _fence: &super::Fence,
    ) -> Result<Option<crate::AcquiredSurfaceTexture<super::Api>>, crate::SurfaceError> {
        // let swapchain = self.swapchain.read();
        // let sc = swapchain.as_ref().unwrap();
        // let texture = super::Texture {
        //     inner: super::TextureInner::Renderbuffer {
        //         raw: sc.renderbuffer,
        //     },
        //     drop_guard: None,
        //     array_layer_count: 1,
        //     mip_level_count: 1,
        //     format: sc.format,
        //     format_desc: sc.format_desc.clone(),
        //     copy_size: crate::CopyExtent {
        //         width: sc.extent.width,
        //         height: sc.extent.height,
        //         depth: 1,
        //     },
        // };
        // Ok(Some(crate::AcquiredSurfaceTexture {
        //     texture,
        //     suboptimal: false,
        // }))
        todo!();
    }
    unsafe fn discard_texture(&self, _texture: super::Texture) {}
}
