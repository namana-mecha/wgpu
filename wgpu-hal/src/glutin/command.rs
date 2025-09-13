use crate::TextureUses;

#[derive(Debug)]
pub struct CommandEncoder;
impl crate::CommandEncoder for CommandEncoder {
    type A = super::Api;

    unsafe fn begin_encoding(&mut self, label: crate::Label) -> Result<(), crate::DeviceError> {
        println!("CommandEncoder::begin_encoding(label: {:?})", label);
        Ok(())
    }

    unsafe fn discard_encoding(&mut self) {
        println!("CommandEncoder::discard_encoding()");
    }

    unsafe fn end_encoding(
        &mut self,
    ) -> Result<<Self::A as crate::Api>::CommandBuffer, crate::DeviceError> {
        println!("CommandEncoder::end_encoding()");
        todo!()
    }

    unsafe fn reset_all<I>(&mut self, command_buffers: I)
    where
        I: Iterator<Item = <Self::A as crate::Api>::CommandBuffer>,
    {
        println!("CommandEncoder::reset_all(command_buffers: ...)");
        todo!()
    }

    unsafe fn transition_buffers<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = crate::BufferBarrier<'a, <Self::A as crate::Api>::Buffer>>,
    {
        println!("CommandEncoder::transition_buffers(barriers: ...)");
    }

    unsafe fn transition_textures<'a, T>(&mut self, barriers: T)
    where
        T: Iterator<Item = crate::TextureBarrier<'a, <Self::A as crate::Api>::Texture>>,
    {
        println!("CommandEncoder::transition_textures(barriers: ...)");
        todo!()
    }

    unsafe fn clear_buffer(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        range: crate::MemoryRange,
    ) {
        println!(
            "CommandEncoder::clear_buffer(buffer: ?, range: {:?})",
            range
        );
    }

    unsafe fn copy_buffer_to_buffer<T>(
        &mut self,
        src: &<Self::A as crate::Api>::Buffer,
        dst: &<Self::A as crate::Api>::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferCopy>,
    {
        println!("CommandEncoder::copy_buffer_to_buffer(src: ?, dst: ?, regions: ...)");
    }

    unsafe fn copy_texture_to_texture<T>(
        &mut self,
        src: &<Self::A as crate::Api>::Texture,
        src_usage: TextureUses,
        dst: &<Self::A as crate::Api>::Texture,
        regions: T,
    ) where
        T: Iterator<Item = crate::TextureCopy>,
    {
        println!(
            "CommandEncoder::copy_texture_to_texture(src: ?, src_usage: {:?}, dst: ?, regions: ...)",
            src_usage
        );
        todo!()
    }

    unsafe fn copy_buffer_to_texture<T>(
        &mut self,
        src: &<Self::A as crate::Api>::Buffer,
        dst: &<Self::A as crate::Api>::Texture,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        println!("CommandEncoder::copy_buffer_to_texture(src: ?, dst: ?, regions: ...)");
        todo!()
    }

    unsafe fn copy_texture_to_buffer<T>(
        &mut self,
        src: &<Self::A as crate::Api>::Texture,
        src_usage: TextureUses,
        dst: &<Self::A as crate::Api>::Buffer,
        regions: T,
    ) where
        T: Iterator<Item = crate::BufferTextureCopy>,
    {
        println!(
            "CommandEncoder::copy_texture_to_buffer(src: ?, src_usage: {:?}, dst: ?, regions: ...)",
            src_usage
        );
        todo!()
    }

    unsafe fn set_bind_group(
        &mut self,
        layout: &<Self::A as crate::Api>::PipelineLayout,
        index: u32,
        group: &<Self::A as crate::Api>::BindGroup,
        dynamic_offsets: &[wgt::DynamicOffset],
    ) {
        println!("CommandEncoder::set_bind_group(layout: ?, index: {:?}, group: ?, dynamic_offsets: {:?})", index, dynamic_offsets);
        todo!()
    }

    unsafe fn set_push_constants(
        &mut self,
        layout: &<Self::A as crate::Api>::PipelineLayout,
        stages: wgt::ShaderStages,
        offset_bytes: u32,
        data: &[u32],
    ) {
        println!("CommandEncoder::set_push_constants(layout: ?, stages: {:?}, offset_bytes: {:?}, data: {:?})", stages, offset_bytes, data);
        todo!()
    }

    unsafe fn insert_debug_marker(&mut self, label: &str) {
        println!("CommandEncoder::insert_debug_marker(label: {:?})", label);
        todo!()
    }

    unsafe fn begin_debug_marker(&mut self, group_label: &str) {
        println!(
            "CommandEncoder::begin_debug_marker(group_label: {:?})",
            group_label
        );
        todo!()
    }

    unsafe fn end_debug_marker(&mut self) {
        println!("CommandEncoder::end_debug_marker()");
        todo!()
    }

    unsafe fn begin_query(&mut self, set: &<Self::A as crate::Api>::QuerySet, index: u32) {
        println!("CommandEncoder::begin_query(set: ?, index: {:?})", index);
        todo!()
    }

    unsafe fn end_query(&mut self, set: &<Self::A as crate::Api>::QuerySet, index: u32) {
        println!("CommandEncoder::end_query(set: ?, index: {:?})", index);
        todo!()
    }

    unsafe fn write_timestamp(&mut self, set: &<Self::A as crate::Api>::QuerySet, index: u32) {
        println!(
            "CommandEncoder::write_timestamp(set: ?, index: {:?})",
            index
        );
        todo!()
    }

    unsafe fn reset_queries(
        &mut self,
        set: &<Self::A as crate::Api>::QuerySet,
        range: core::ops::Range<u32>,
    ) {
        println!("CommandEncoder::reset_queries(set: ?, range: {:?})", range);
        todo!()
    }

    unsafe fn copy_query_results(
        &mut self,
        set: &<Self::A as crate::Api>::QuerySet,
        range: core::ops::Range<u32>,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        stride: wgt::BufferSize,
    ) {
        println!("CommandEncoder::copy_query_results(set: ?, range: {:?}, buffer: ?, offset: {:?}, stride: {:?})", range, offset, stride);
        todo!()
    }

    unsafe fn begin_render_pass(
        &mut self,
        desc: &crate::RenderPassDescriptor<
            <Self::A as crate::Api>::QuerySet,
            <Self::A as crate::Api>::TextureView,
        >,
    ) {
        println!("CommandEncoder::begin_render_pass(desc: ?)");
        todo!()
    }

    unsafe fn end_render_pass(&mut self) {
        println!("CommandEncoder::end_render_pass()");
        todo!()
    }

    unsafe fn set_render_pipeline(&mut self, pipeline: &<Self::A as crate::Api>::RenderPipeline) {
        println!("CommandEncoder::set_render_pipeline(pipeline: ?)");
        todo!()
    }

    unsafe fn set_index_buffer<'a>(
        &mut self,
        binding: crate::BufferBinding<'a, <Self::A as crate::Api>::Buffer>,
        format: wgt::IndexFormat,
    ) {
        println!(
            "CommandEncoder::set_index_buffer(binding: ?, format: {:?})",
            format
        );
        todo!()
    }

    unsafe fn set_vertex_buffer<'a>(
        &mut self,
        index: u32,
        binding: crate::BufferBinding<'a, <Self::A as crate::Api>::Buffer>,
    ) {
        println!(
            "CommandEncoder::set_vertex_buffer(index: {:?}, binding: ?)",
            index
        );
        todo!()
    }

    unsafe fn set_viewport(&mut self, rect: &crate::Rect<f32>, depth_range: core::ops::Range<f32>) {
        println!(
            "CommandEncoder::set_viewport(rect: {:?}, depth_range: {:?})",
            rect, depth_range
        );
        todo!()
    }

    unsafe fn set_scissor_rect(&mut self, rect: &crate::Rect<u32>) {
        println!("CommandEncoder::set_scissor_rect(rect: {:?})", rect);
        todo!()
    }

    unsafe fn set_stencil_reference(&mut self, value: u32) {
        println!("CommandEncoder::set_stencil_reference(value: {:?})", value);
        todo!()
    }

    unsafe fn set_blend_constants(&mut self, color: &[f32; 4]) {
        println!("CommandEncoder::set_blend_constants(color: {:?})", color);
        todo!()
    }

    unsafe fn draw(
        &mut self,
        first_vertex: u32,
        vertex_count: u32,
        first_instance: u32,
        instance_count: u32,
    ) {
        println!("CommandEncoder::draw(first_vertex: {:?}, vertex_count: {:?}, first_instance: {:?}, instance_count: {:?})", first_vertex, vertex_count, first_instance, instance_count);
        todo!()
    }

    unsafe fn draw_indexed(
        &mut self,
        first_index: u32,
        index_count: u32,
        base_vertex: i32,
        first_instance: u32,
        instance_count: u32,
    ) {
        println!("CommandEncoder::draw_indexed(first_index: {:?}, index_count: {:?}, base_vertex: {:?}, first_instance: {:?}, instance_count: {:?})", first_index, index_count, base_vertex, first_instance, instance_count);
        todo!()
    }

    unsafe fn draw_indirect(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        println!(
            "CommandEncoder::draw_indirect(buffer: ?, offset: {:?}, draw_count: {:?})",
            offset, draw_count
        );
        todo!()
    }

    unsafe fn draw_indexed_indirect(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        draw_count: u32,
    ) {
        println!(
            "CommandEncoder::draw_indexed_indirect(buffer: ?, offset: {:?}, draw_count: {:?})",
            offset, draw_count
        );
        todo!()
    }

    unsafe fn draw_indirect_count(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &<Self::A as crate::Api>::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        println!("CommandEncoder::draw_indirect_count(buffer: ?, offset: {:?}, count_buffer: ?, count_offset: {:?}, max_count: {:?})", offset, count_offset, max_count);
        todo!()
    }

    unsafe fn draw_indexed_indirect_count(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
        count_buffer: &<Self::A as crate::Api>::Buffer,
        count_offset: wgt::BufferAddress,
        max_count: u32,
    ) {
        println!("CommandEncoder::draw_indexed_indirect_count(buffer: ?, offset: {:?}, count_buffer: ?, count_offset: {:?}, max_count: {:?})", offset, count_offset, max_count);
        todo!()
    }

    unsafe fn begin_compute_pass(
        &mut self,
        desc: &crate::ComputePassDescriptor<<Self::A as crate::Api>::QuerySet>,
    ) {
        println!("CommandEncoder::begin_compute_pass(desc: ?)");
        todo!()
    }

    unsafe fn end_compute_pass(&mut self) {
        println!("CommandEncoder::end_compute_pass()");
        todo!()
    }

    unsafe fn set_compute_pipeline(&mut self, pipeline: &<Self::A as crate::Api>::ComputePipeline) {
        println!("CommandEncoder::set_compute_pipeline(pipeline: ?)");
        todo!()
    }

    unsafe fn dispatch(&mut self, count: [u32; 3]) {
        println!("CommandEncoder::dispatch(count: {:?})", count);
        todo!()
    }

    unsafe fn dispatch_indirect(
        &mut self,
        buffer: &<Self::A as crate::Api>::Buffer,
        offset: wgt::BufferAddress,
    ) {
        println!(
            "CommandEncoder::dispatch_indirect(buffer: ?, offset: {:?})",
            offset
        );
        todo!()
    }

    unsafe fn build_acceleration_structures<'a, T>(&mut self, descriptor_count: u32, descriptors: T)
    where
        Self::A: 'a,
        T: IntoIterator<
            Item = crate::BuildAccelerationStructureDescriptor<
                'a,
                <Self::A as crate::Api>::Buffer,
                <Self::A as crate::Api>::AccelerationStructure,
            >,
        >,
    {
        println!("CommandEncoder::build_acceleration_structures(descriptor_count: {:?}, descriptors: ...)", descriptor_count);
        todo!()
    }

    unsafe fn place_acceleration_structure_barrier(
        &mut self,
        barrier: crate::AccelerationStructureBarrier,
    ) {
        println!("CommandEncoder::place_acceleration_structure_barrier(barrier: ?)");
        todo!()
    }
}

#[derive(Debug)]
pub struct CommandBuffer;
impl crate::DynCommandBuffer for CommandBuffer {}
