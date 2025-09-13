use wgt::{CompositeAlphaMode, PresentMode, TextureFormat};

use crate::TextureUses;

pub struct Adapter;
impl crate::Adapter for Adapter {
    type A = super::Api;

    unsafe fn open(
        &self,
        features: wgt::Features,
        limits: &wgt::Limits,
        memory_hints: &wgt::MemoryHints,
    ) -> Result<crate::OpenDevice<Self::A>, crate::DeviceError> {
        println!(
            "Adapter::open(features: {:?}, limits: {:?}, memory_hints: {:?})",
            features, limits, memory_hints
        );
        Ok(crate::OpenDevice {
            device: super::Device {},
            queue: super::Queue {},
        })
    }

    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> crate::TextureFormatCapabilities {
        println!("Adapter::texture_format_capabilities(format: {:?})", format);
        todo!()
    }

    unsafe fn surface_capabilities(
        &self,
        surface: &<Self::A as crate::Api>::Surface,
    ) -> Option<crate::SurfaceCapabilities> {
        // Note: The 'surface' parameter's type doesn't guarantee `Debug` implementation,
        // so we're printing a placeholder for it.
        println!("Adapter::surface_capabilities(surface: ?)");
        Some(crate::SurfaceCapabilities {
            formats: vec![TextureFormat::R8Unorm],
            maximum_frame_latency: 0..=20,
            current_extent: wgt::Extent3d {
                width: 1000,
                height: 1000,
                depth_or_array_layers: 1,
            }
            .into(),
            usage: TextureUses::COLOR_TARGET,
            present_modes: vec![PresentMode::AutoVsync],
            composite_alpha_modes: vec![CompositeAlphaMode::Auto],
        })
    }

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        println!("Adapter::get_presentation_timestamp()");
        todo!()
    }
}
