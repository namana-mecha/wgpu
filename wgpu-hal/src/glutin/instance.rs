use core::num::NonZero;
use std::vec::Vec;

use glutin::{
    config::ConfigTemplateBuilder,
    display::{Display, DisplayApiPreference},
    prelude::GlDisplay,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use parking_lot::lock_api::Mutex;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle, WaylandDisplayHandle};
use wayland_client::Connection;
use wgt::{DownlevelCapabilities, Limits};

pub struct Instance {}
impl crate::Instance for Instance {
    type A = super::Api;

    unsafe fn init(desc: &crate::InstanceDescriptor) -> Result<Self, crate::InstanceError> {
        println!("Instance::init(desc: {:?})", desc);
        Ok(Self {})
    }

    unsafe fn create_surface(
        &self,
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
    ) -> Result<<Self::A as crate::Api>::Surface, crate::InstanceError> {
        println!("Instance::create_surface(display_handle: ?, window_handle: ?)");
        let display = unsafe {
            Display::new(display_handle, DisplayApiPreference::Egl)
                .expect("couldn't create wayland display")
        };
        let template = ConfigTemplateBuilder::new();
        let configs = unsafe {
            display
                .find_configs(template.build())
                .expect("couldn't find surface configs")
        };
        let config = configs.last().expect("no supported config found");
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle,
            NonZero::<u32>::new(1000).unwrap(),
            NonZero::<u32>::new(1000).unwrap(),
        );
        let surface = unsafe {
            display
                .create_window_surface(&config, &surface_attributes)
                .expect("couldn't create window surface")
        };
        Ok(super::Surface {})
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&<Self::A as crate::Api>::Surface>,
    ) -> Vec<crate::ExposedAdapter<Self::A>> {
        println!("Instance::enumerate_adapters(_surface_hint: ?)");
        let exposed_adapter = crate::ExposedAdapter {
            adapter: super::Adapter {},
            info: wgt::AdapterInfo {
                name: Default::default(),
                vendor: Default::default(),
                device: Default::default(),
                device_type: wgt::DeviceType::IntegratedGpu,
                driver: Default::default(),
                driver_info: Default::default(),
                backend: wgt::Backend::Glutin,
            },
            features: wgt::Features::all_native_mask(),
            capabilities: crate::Capabilities {
                limits: Limits::default(),
                alignments: crate::Alignments {
                    buffer_copy_offset: NonZero::<u64>::new(128).unwrap(),
                    buffer_copy_pitch: NonZero::<u64>::new(128).unwrap(),
                    uniform_bounds_check_alignment: NonZero::<u64>::new(128).unwrap(),
                    raw_tlas_instance_size: Default::default(),
                    ray_tracing_scratch_buffer_alignment: Default::default(),
                },
                downlevel: DownlevelCapabilities::default(),
            },
        };
        vec![exposed_adapter]
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}
