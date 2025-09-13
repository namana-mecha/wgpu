use glow::HasContext;
use glutin::{
    config::{ConfigSurfaceTypes, ConfigTemplateBuilder, GlConfig},
    display::{self, GetGlDisplay},
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{PbufferSurface, SurfaceAttributesBuilder, WindowSurface},
};
use parking_lot::MutexGuard;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WindowHandle};
use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    num::{NonZero, NonZeroI32, NonZeroU32},
    os::raw,
    sync::{Arc, Mutex, RwLock},
    time::Duration,
    vec::Vec,
};
use wgt::{AdapterInfo, Backend, Features, Gles3MinorVersion, SurfaceCapabilities};

use crate::{
    glutin::{AdapterShared, Api},
    Alignments, Capabilities, ExposedAdapter,
};

pub struct AdapterContext {
    pub gl: Mutex<ManuallyDrop<glow::Context>>,
}
impl AdapterContext {
    pub fn new(gl: glow::Context) -> Arc<Self> {
        Arc::new(Self {
            gl: Mutex::new(ManuallyDrop::new(gl)),
        })
    }
}

// struct EglContextLock<'a> {
//     // instance: &'a Arc<EglInstance>,
//     display: khronos_egl::Display,
// }

pub struct AdapterContextLock<'a> {
    glow: MutexGuard<'a, ManuallyDrop<glow::Context>>,
    // egl: Option<EglContextLock<'a>>,
}

impl<'a> std::ops::Deref for AdapterContextLock<'a> {
    type Target = glow::Context;

    fn deref(&self) -> &Self::Target {
        &self.glow
    }
}

impl<'a> Drop for AdapterContextLock<'a> {
    fn drop(&mut self) {
        // if let Some(egl) = self.egl.take() {
        //     egl.instance
        //         .make_current(egl.display, None, None, None)
        //         .unwrap();
        // }
    }
}

#[derive(Debug)]
struct Inner {
    #[allow(unused)]
    version: (i32, i32),
    supports_native_window: bool,
    config: glutin::api::egl::config::Config,
    display: glutin::api::egl::display::Display,
}

pub struct Instance {
    flags: wgt::InstanceFlags,
    inner: Mutex<Inner>,
}

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

        let template_builder = ConfigTemplateBuilder::new().prefer_hardware_accelerated(Some(true));
        let template = template_builder.build();
        let config = unsafe {
            display
                .find_configs(template)
                .expect("couldn't find configs")
                .next()
                .unwrap()
        };
        let inner = Inner {
            display: display,
            supports_native_window: true,
            version: (3, 0),
            config: config,
        };
        Ok(Self {
            flags: desc.flags,
            inner: Mutex::new(inner),
        })
    }

    unsafe fn create_surface(
        &self,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> Result<<Self::A as crate::Api>::Surface, crate::InstanceError> {
        log::error!("Instance::create_surface(display_handle: ?, window_handle: ?)");
        let inner = self.inner.lock().unwrap();
        let display = glutin::display::Display::Egl(inner.display.clone());
        let config = glutin::config::Config::Egl(inner.config.clone());

        let surface_attributes_builder = SurfaceAttributesBuilder::<WindowSurface>::new();
        let surface_attributes = surface_attributes_builder.build(
            window_handle,
            NonZero::new(1280).unwrap(),
            NonZero::new(720).unwrap(),
        );
        unsafe { display.create_window_surface(&config, &surface_attributes) };

        Ok(Surface {
            config: inner.config.clone(),
            presentable: inner.supports_native_window,
            raw_window_handle: window_handle,
            swapchain: RwLock::new(None),
        })
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&<Self::A as crate::Api>::Surface>,
    ) -> Vec<crate::ExposedAdapter<Self::A>> {
        let context_attributes_builder = glutin::context::ContextAttributesBuilder::default();
        let context_attributes = context_attributes_builder.build(None);
        let inner = self.inner.lock().expect("couldn't aquire lock");
        let not_current_context = unsafe {
            inner
                .display
                .create_context(&inner.config, &context_attributes)
                .expect("couldn't create context")
        };
        let current_context = unsafe {
            not_current_context
                .make_current_surfaceless()
                .expect("couldn't make current")
        };
        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                inner.display.get_proc_address(&CString::new(s).expect(s)) as *const _
            })
        };
        let _ = current_context.make_not_current();

        // vec![crate::ExposedAdapter {
        //     info: AdapterInfo {
        //         name: format!("{:?}", inner.config.api()),
        //         vendor: Default::default(),
        //         device: Default::default(),
        //         device_type: wgt::DeviceType::IntegratedGpu,
        //         driver: "etnaviv".into(),
        //         driver_info: "something".into(),
        //         backend: Backend::Glutin,
        //     },
        //     features: wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
        //     capabilities: Capabilities {
        //         limits: wgt::Limits::default(),
        //         alignments: Alignments {
        //             buffer_copy_offset: NonZero::new(1).unwrap(),
        //             buffer_copy_pitch: NonZero::new(1).unwrap(),
        //             uniform_bounds_check_alignment: NonZero::new(1).unwrap(),
        //             raw_tlas_instance_size: Default::default(),
        //             ray_tracing_scratch_buffer_alignment: Default::default(),
        //         },
        //         downlevel: wgt::DownlevelCapabilities::default(),
        //     },
        //     adapter: super::Adapter::expose(AdapterContext { gl: Mutex::new(gl) }),
        // }]
        unsafe {
            super::Adapter::expose(AdapterContext {
                gl: Mutex::new(ManuallyDrop::new(gl)),
            })
        }
        .into_iter()
        .collect()
    }
}

#[derive(Debug)]
pub struct Swapchain {
    surface: glutin::surface::Surface<WindowSurface>,
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
    // DONE
    // egl: EglContext,
    // wsi: WindowSystemInterface,
    config: glutin::api::egl::config::Config,
    pub(super) presentable: bool,
    raw_window_handle: raw_window_handle::RawWindowHandle,
    swapchain: RwLock<Option<Swapchain>>,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}

impl Surface {
    pub(super) unsafe fn present(
        &self,
        _suf_texture: super::Texture,
        context: &AdapterContext,
    ) -> Result<(), crate::SurfaceError> {
        let gl = unsafe { context.gl.lock().unwrap() };
        let swapchain = self.swapchain.read().unwrap();
        let sc = swapchain.as_ref().unwrap();

        unsafe { gl.disable(glow::SCISSOR_TEST) };
        unsafe { gl.color_mask(true, true, true, true) };

        unsafe { gl.bind_framebuffer(glow::DRAW_FRAMEBUFFER, None) };
        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(sc.framebuffer)) };

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

        unsafe { gl.bind_framebuffer(glow::READ_FRAMEBUFFER, None) };

        // self.egl
        //     .instance
        //     .swap_buffers(self.egl.display, sc.surface)
        //     .map_err(|e| {
        //         log::error!("swap_buffers failed: {}", e);
        //         crate::SurfaceError::Lost
        //         // TODO: should we unset the current context here?
        //     })?;
        // self.egl
        //     .instance
        //     .make_current(self.egl.display, None, None, None)
        //     .map_err(|e| {
        //         log::error!("make_current(null) failed: {}", e);
        //         crate::SurfaceError::Lost
        //     })?;

        Ok(())
    }

    unsafe fn unconfigure_impl(
        &self,
        device: &super::Device,
    ) -> Option<(
        glutin::surface::Surface<WindowSurface>,
        Option<*mut raw::c_void>,
    )> {
        let gl = &device.shared.context.gl.lock().unwrap();
        match self.swapchain.write().unwrap().take() {
            Some(sc) => {
                unsafe { gl.delete_renderbuffer(sc.renderbuffer) };
                unsafe { gl.delete_framebuffer(sc.framebuffer) };
                Some((sc.surface, sc.wl_window))
            }
            None => None,
        }
    }

    pub fn supports_srgb(&self) -> bool {
        true
    }
}

impl crate::Surface for Surface {
    type A = super::Api;

    unsafe fn configure(
        &self,
        device: &super::Device,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        // TODO
        Ok(())
    }

    unsafe fn unconfigure(&self, device: &super::Device) {
        // if let Some((surface, wl_window)) = unsafe { self.unconfigure_impl(device) } {
        //     // self.egl
        //     //     .instance
        //     //     .destroy_surface(self.egl.display, surface)
        //     //     .unwrap();

        //     // TODO: understand
        //     // if let Some(window) = wl_window {
        //     //     let library = &self
        //     //         .wsi
        //     //         .display_owner
        //     //         .as_ref()
        //     //         .expect("unsupported window")
        //     //         .library;
        //     //     let wl_egl_window_destroy: libloading::Symbol<WlEglWindowDestroyFun> =
        //     //         unsafe { library.get(b"wl_egl_window_destroy\0") }.unwrap();
        //     //     unsafe { wl_egl_window_destroy(window) };
        //     // }
        // }
    }

    unsafe fn acquire_texture(
        &self,
        _timeout_ms: Option<Duration>, //TODO
        _fence: &super::Fence,
    ) -> Result<Option<crate::AcquiredSurfaceTexture<super::Api>>, crate::SurfaceError> {
        let swapchain = self.swapchain.read().unwrap();
        let sc = swapchain.as_ref().unwrap();
        let texture = super::Texture {
            inner: super::TextureInner::Renderbuffer {
                raw: sc.renderbuffer,
            },
            drop_guard: None,
            array_layer_count: 1,
            mip_level_count: 1,
            format: sc.format,
            format_desc: sc.format_desc.clone(),
            copy_size: crate::CopyExtent {
                width: sc.extent.width,
                height: sc.extent.height,
                depth: 1,
            },
        };
        Ok(Some(crate::AcquiredSurfaceTexture {
            texture,
            suboptimal: false,
        }))
    }
    unsafe fn discard_texture(&self, _texture: super::Texture) {}
}
