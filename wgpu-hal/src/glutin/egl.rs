use glutin::{
    config::{ConfigSurfaceTypes, ConfigTemplateBuilder, GlConfig},
    display,
    prelude::{GlDisplay, NotCurrentGlContext, PossiblyCurrentGlContext},
    surface::{PbufferSurface, SurfaceAttributesBuilder},
};
use parking_lot::MutexGuard;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use std::{
    ffi::{CStr, CString},
    mem::ManuallyDrop,
    num::{NonZero, NonZeroI32, NonZeroU32},
    sync::{Arc, Mutex},
    vec::Vec,
};
use wgt::{AdapterInfo, Backend, Features, Gles3MinorVersion, SurfaceCapabilities};

use crate::{glutin::Api, Alignments, Capabilities, ExposedAdapter};

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
    display: glutin::api::egl::display::Display,
}

pub struct Instance {
    inner: Mutex<Inner>,
}

impl crate::Instance for Instance {
    type A = super::Api;

    unsafe fn init(desc: &crate::InstanceDescriptor) -> Result<Self, crate::InstanceError> {
        println!("Instance::init(desc: {:?})", desc);
        let device = glutin::api::egl::device::Device::query_devices()
            .unwrap()
            .next()
            .unwrap();
        let display = unsafe {
            glutin::api::egl::display::Display::new(
                // &device, None,
                desc.display
                    .expect("cannot create glutin instance without raw display handle")
                    .as_raw(),
            )
            .expect("couldn't create glutin display")
            // .expect("couldn't create glutin display")
            // glutin::display::Display::new(
            //     desc.display
            //         .expect("cannot create glutin instance without raw display handle")
            //         .as_raw(),
            //     glutin::display::DisplayApiPreference::Egl,
            // )
            // .expect("couldn't create glutin display")
        };

        let inner = Inner { display: (display) };
        Ok(Self {
            inner: Mutex::new(inner),
        })
    }

    unsafe fn create_surface(
        &self,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> Result<<Self::A as crate::Api>::Surface, crate::InstanceError> {
        log::error!("Instance::create_surface(display_handle: ?, window_handle: ?)");
        Ok(super::Surface {})
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&<Self::A as crate::Api>::Surface>,
    ) -> Vec<crate::ExposedAdapter<Self::A>> {
        let template_builder = ConfigTemplateBuilder::new().prefer_hardware_accelerated(Some(true));
        let template = template_builder.build();
        let mut output = vec![];
        let inner = self.inner.lock().expect("couldn't aquire lock");
        for config in unsafe {
            inner
                .display
                .find_configs(template.clone())
                .expect("couldn't find configs")
        } {
            println!("{:?}", config.config_surface_types());
        }
        let config = unsafe {
            inner
                .display
                .find_configs(template)
                .expect("couldn't find configs")
                .next()
                .unwrap()
        };
        let context_attributes_builder = glutin::context::ContextAttributesBuilder::default()
            .with_debug(true)
            .with_context_api(glutin::context::ContextApi::Gles(None));
        let context_attributes = context_attributes_builder.build(None);
        let not_current_context = unsafe {
            inner
                .display
                .create_context(&config, &context_attributes)
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

        output.push(crate::ExposedAdapter {
            info: AdapterInfo {
                name: format!("{:?}", config.api()),
                vendor: Default::default(),
                device: Default::default(),
                device_type: wgt::DeviceType::IntegratedGpu,
                driver: "etnaviv".into(),
                driver_info: "something".into(),
                backend: Backend::Glutin,
            },
            features: wgt::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            capabilities: Capabilities {
                limits: wgt::Limits::default(),
                alignments: Alignments {
                    buffer_copy_offset: NonZero::new(1).unwrap(),
                    buffer_copy_pitch: NonZero::new(1).unwrap(),
                    uniform_bounds_check_alignment: NonZero::new(1).unwrap(),
                    raw_tlas_instance_size: Default::default(),
                    ray_tracing_scratch_buffer_alignment: Default::default(),
                },
                downlevel: wgt::DownlevelCapabilities::default(),
            },
            adapter: super::Adapter {
                config: Arc::new(config),
                context: super::AdapterContext::new(gl),
            },
        });
        output
    }
}

#[derive(Debug)]
pub struct Surface {
    // TODO
    // egl: EglContext,
    // wsi: WindowSystemInterface,
    // config: khronos_egl::Config,
    // pub(super) presentable: bool,
    // raw_window_handle: raw_window_handle::RawWindowHandle,
    // swapchain: RwLock<Option<Swapchain>>,
    // srgb_kind: SrgbFrameBufferKind,
}

unsafe impl Send for Surface {}
unsafe impl Sync for Surface {}


impl Surface {

}

impl crate::Surface for Surface {
    type A = super::Api;

    unsafe fn configure(
        &self,
        device: &<Self::A as crate::Api>::Device,
        config: &crate::SurfaceConfiguration,
    ) -> Result<(), crate::SurfaceError> {
        todo!()
    }

    unsafe fn unconfigure(&self, device: &<Self::A as crate::Api>::Device) {
        todo!()
    }

    unsafe fn acquire_texture(
        &self,
        timeout: Option<std::time::Duration>,
        fence: &<Self::A as crate::Api>::Fence,
    ) -> Result<Option<crate::AcquiredSurfaceTexture<Self::A>>, crate::SurfaceError> {
        todo!()
    }

    unsafe fn discard_texture(&self, texture: <Self::A as crate::Api>::SurfaceTexture) {
        todo!()
    }
}