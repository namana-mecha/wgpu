use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use std::vec::Vec;

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
        Ok(super::Surface {})
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&<Self::A as crate::Api>::Surface>,
    ) -> Vec<crate::ExposedAdapter<Self::A>> {
        println!("Instance::enumerate_adapters(_surface_hint: ?)");
        vec![]
    }
}

unsafe impl Send for Instance {}
unsafe impl Sync for Instance {}
