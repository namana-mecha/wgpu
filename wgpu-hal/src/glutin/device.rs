use core::ptr::NonNull;
use std::sync::{Arc, Mutex};

use crate::AtomicFenceValue;

use super::adapter::AdapterContext;

pub struct Device {
    pub context: Arc<AdapterContext>,
    pub counters: Arc<wgt::HalCounters>,
}
impl crate::Device for Device {
    type A = super::Api;

    unsafe fn create_buffer(
        &self,
        desc: &crate::BufferDescriptor,
    ) -> Result<<Self::A as crate::Api>::Buffer, crate::DeviceError> {
        println!("Device::create_buffer(desc: {:?})", desc);
        Ok(super::Buffer { size: desc.size })
    }

    unsafe fn destroy_buffer(&self, buffer: <Self::A as crate::Api>::Buffer) {
        println!("Device::destroy_buffer(buffer: ?)");
    }

    unsafe fn add_raw_buffer(&self, buffer: &<Self::A as crate::Api>::Buffer) {
        println!("Device::add_raw_buffer(buffer: ?)");

        todo!()
    }

    unsafe fn map_buffer(
        &self,
        buffer: &<Self::A as crate::Api>::Buffer,
        range: crate::MemoryRange,
    ) -> Result<crate::BufferMapping, crate::DeviceError> {
        println!("Device::map_buffer(buffer: ?, range: {:?})", range);
        let mut mapping = vec![];
        for x in range {
            for _ in 0..buffer.size {
                mapping.extend(vec![x as u8; 100]);
            }
        }
        Ok(crate::BufferMapping {
            ptr: NonNull::new(mapping.as_mut_ptr()).unwrap(),
            is_coherent: false,
        })
    }

    unsafe fn unmap_buffer(&self, buffer: &<Self::A as crate::Api>::Buffer) {
        println!("Device::unmap_buffer(buffer: ?)");
    }

    unsafe fn flush_mapped_ranges<I>(&self, buffer: &<Self::A as crate::Api>::Buffer, ranges: I)
    where
        I: Iterator<Item = crate::MemoryRange>,
    {
        println!("Device::flush_mapped_ranges(buffer: ?, ranges: ...)");
    }

    unsafe fn invalidate_mapped_ranges<I>(
        &self,
        buffer: &<Self::A as crate::Api>::Buffer,
        ranges: I,
    ) where
        I: Iterator<Item = crate::MemoryRange>,
    {
        println!("Device::invalidate_mapped_ranges(buffer: ?, ranges: ...)");
        todo!()
    }

    unsafe fn create_texture(
        &self,
        desc: &crate::TextureDescriptor,
    ) -> Result<<Self::A as crate::Api>::Texture, crate::DeviceError> {
        println!("Device::create_texture(desc: {:?})", desc);
        Ok(super::Texture {})
    }

    unsafe fn destroy_texture(&self, texture: <Self::A as crate::Api>::Texture) {
        println!("Device::destroy_texture(texture: ?)");
    }

    unsafe fn add_raw_texture(&self, texture: &<Self::A as crate::Api>::Texture) {
        println!("Device::add_raw_texture(texture: ?)");
        todo!()
    }

    unsafe fn create_texture_view(
        &self,
        texture: &<Self::A as crate::Api>::Texture,
        desc: &crate::TextureViewDescriptor,
    ) -> Result<<Self::A as crate::Api>::TextureView, crate::DeviceError> {
        println!("Device::create_texture_view(texture: ?, desc: {:?})", desc);
        Ok(super::TextureView {})
    }

    unsafe fn destroy_texture_view(&self, view: <Self::A as crate::Api>::TextureView) {
        println!("Device::destroy_texture_view(view: ?)");
    }

    unsafe fn create_sampler(
        &self,
        desc: &crate::SamplerDescriptor,
    ) -> Result<<Self::A as crate::Api>::Sampler, crate::DeviceError> {
        println!("Device::create_sampler(desc: {:?})", desc);
        Ok(super::Sampler {})
    }

    unsafe fn destroy_sampler(&self, sampler: <Self::A as crate::Api>::Sampler) {
        println!("Device::destroy_sampler(sampler: ?)");
    }

    unsafe fn create_command_encoder(
        &self,
        desc: &crate::CommandEncoderDescriptor<<Self::A as crate::Api>::Queue>,
    ) -> Result<<Self::A as crate::Api>::CommandEncoder, crate::DeviceError> {
        println!("Device::create_command_encoder(desc: ?)");
        Ok(super::CommandEncoder)
    }

    unsafe fn create_bind_group_layout(
        &self,
        desc: &crate::BindGroupLayoutDescriptor,
    ) -> Result<<Self::A as crate::Api>::BindGroupLayout, crate::DeviceError> {
        println!("Device::create_bind_group_layout(desc: {:?})", desc);
        Ok(super::BindGroupLayout {})
    }

    unsafe fn destroy_bind_group_layout(
        &self,
        bg_layout: <Self::A as crate::Api>::BindGroupLayout,
    ) {
        println!("Device::destroy_bind_group_layout(bg_layout: ?)");
    }

    unsafe fn create_pipeline_layout(
        &self,
        desc: &crate::PipelineLayoutDescriptor<<Self::A as crate::Api>::BindGroupLayout>,
    ) -> Result<<Self::A as crate::Api>::PipelineLayout, crate::DeviceError> {
        println!("Device::create_pipeline_layout(desc: ?)");
        Ok(super::PipelineLayout {})
    }

    unsafe fn destroy_pipeline_layout(
        &self,
        pipeline_layout: <Self::A as crate::Api>::PipelineLayout,
    ) {
        println!("Device::destroy_pipeline_layout(pipeline_layout: ?)");
    }

    unsafe fn create_bind_group(
        &self,
        desc: &crate::BindGroupDescriptor<
            <Self::A as crate::Api>::BindGroupLayout,
            <Self::A as crate::Api>::Buffer,
            <Self::A as crate::Api>::Sampler,
            <Self::A as crate::Api>::TextureView,
            <Self::A as crate::Api>::AccelerationStructure,
        >,
    ) -> Result<<Self::A as crate::Api>::BindGroup, crate::DeviceError> {
        println!("Device::create_bind_group(desc: ?)");
        Ok(super::BindGroup {})
    }

    unsafe fn destroy_bind_group(&self, group: <Self::A as crate::Api>::BindGroup) {
        println!("Device::destroy_bind_group(group: ?)");
    }

    unsafe fn create_shader_module(
        &self,
        desc: &crate::ShaderModuleDescriptor,
        shader: crate::ShaderInput,
    ) -> Result<<Self::A as crate::Api>::ShaderModule, crate::ShaderError> {
        println!("Device::create_shader_module(desc: ?, shader: ?)");
        Ok(super::ShaderModule {})
    }

    unsafe fn destroy_shader_module(&self, module: <Self::A as crate::Api>::ShaderModule) {
        println!("Device::destroy_shader_module(module: ?)");
    }

    unsafe fn create_render_pipeline(
        &self,
        desc: &crate::RenderPipelineDescriptor<
            <Self::A as crate::Api>::PipelineLayout,
            <Self::A as crate::Api>::ShaderModule,
            <Self::A as crate::Api>::PipelineCache,
        >,
    ) -> Result<<Self::A as crate::Api>::RenderPipeline, crate::PipelineError> {
        println!("Device::create_render_pipeline(desc: ?)");
        todo!()
    }

    unsafe fn destroy_render_pipeline(&self, pipeline: <Self::A as crate::Api>::RenderPipeline) {
        println!("Device::destroy_render_pipeline(pipeline: ?)");
        todo!()
    }

    unsafe fn create_compute_pipeline(
        &self,
        desc: &crate::ComputePipelineDescriptor<
            <Self::A as crate::Api>::PipelineLayout,
            <Self::A as crate::Api>::ShaderModule,
            <Self::A as crate::Api>::PipelineCache,
        >,
    ) -> Result<<Self::A as crate::Api>::ComputePipeline, crate::PipelineError> {
        println!("Device::create_compute_pipeline(desc: ?)");
        Ok(super::ComputePipeline {})
    }

    unsafe fn destroy_compute_pipeline(&self, pipeline: <Self::A as crate::Api>::ComputePipeline) {
        println!("Device::destroy_compute_pipeline(pipeline: ?)");
    }

    unsafe fn create_pipeline_cache(
        &self,
        desc: &crate::PipelineCacheDescriptor<'_>,
    ) -> Result<<Self::A as crate::Api>::PipelineCache, crate::PipelineCacheError> {
        println!("Device::create_pipeline_cache(desc: ?)");
        todo!()
    }

    unsafe fn destroy_pipeline_cache(&self, cache: <Self::A as crate::Api>::PipelineCache) {
        println!("Device::destroy_pipeline_cache(cache: ?)");
        todo!()
    }

    unsafe fn create_query_set(
        &self,
        desc: &wgt::QuerySetDescriptor<crate::Label>,
    ) -> Result<<Self::A as crate::Api>::QuerySet, crate::DeviceError> {
        println!("Device::create_query_set(desc: {:?})", desc);
        todo!()
    }

    unsafe fn destroy_query_set(&self, set: <Self::A as crate::Api>::QuerySet) {
        println!("Device::destroy_query_set(set: ?)");
        todo!()
    }

    unsafe fn create_fence(&self) -> Result<<Self::A as crate::Api>::Fence, crate::DeviceError> {
        self.counters.fences.add(1);
        Ok(super::Fence {
            last_completed: AtomicFenceValue::new(0),
            pending: Vec::new(),
        })
    }

    unsafe fn destroy_fence(&self, fence: <Self::A as crate::Api>::Fence) {
        println!("Device::destroy_fence(fence: ?)");
    }

    unsafe fn get_fence_value(
        &self,
        fence: &<Self::A as crate::Api>::Fence,
    ) -> Result<crate::FenceValue, crate::DeviceError> {
        println!("Device::get_fence_value(fence: ?)");
        todo!()
    }

    unsafe fn wait(
        &self,
        fence: &<Self::A as crate::Api>::Fence,
        value: crate::FenceValue,
        timeout_ms: u32,
    ) -> Result<bool, crate::DeviceError> {
        println!(
            "Device::wait(fence: ?, value: {:?}, timeout_ms: {:?})",
            value, timeout_ms
        );
        Ok(true)
    }

    unsafe fn create_acceleration_structure(
        &self,
        desc: &crate::AccelerationStructureDescriptor,
    ) -> Result<<Self::A as crate::Api>::AccelerationStructure, crate::DeviceError> {
        println!("Device::create_acceleration_structure(desc: {:?})", desc);
        todo!()
    }

    unsafe fn get_acceleration_structure_build_sizes(
        &self,
        desc: &crate::GetAccelerationStructureBuildSizesDescriptor<<Self::A as crate::Api>::Buffer>,
    ) -> crate::AccelerationStructureBuildSizes {
        println!("Device::get_acceleration_structure_build_sizes(desc: ?)");
        todo!()
    }

    unsafe fn get_acceleration_structure_device_address(
        &self,
        acceleration_structure: &<Self::A as crate::Api>::AccelerationStructure,
    ) -> wgt::BufferAddress {
        println!("Device::get_acceleration_structure_device_address(acceleration_structure: ?)");
        todo!()
    }

    unsafe fn destroy_acceleration_structure(
        &self,
        acceleration_structure: <Self::A as crate::Api>::AccelerationStructure,
    ) {
        println!("Device::destroy_acceleration_structure(acceleration_structure: ?)");
        todo!()
    }

    fn tlas_instance_to_bytes(&self, instance: crate::TlasInstance) -> std::vec::Vec<u8> {
        println!("Device::tlas_instance_to_bytes(instance: ?)");
        todo!()
    }

    fn get_internal_counters(&self) -> wgt::HalCounters {
        println!("Device::get_internal_counters()");
        todo!()
    }

    unsafe fn start_capture(&self) -> bool {
        todo!()
    }

    unsafe fn stop_capture(&self) {
        todo!()
    }
}
