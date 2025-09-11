pub struct Queue;
impl crate::Queue for Queue {
    type A = super::Api;

    unsafe fn submit(
        &self,
        command_buffers: &[&<Self::A as crate::Api>::CommandBuffer],
        surface_textures: &[&<Self::A as crate::Api>::SurfaceTexture],
        signal_fence: (&mut <Self::A as crate::Api>::Fence, crate::FenceValue),
    ) -> Result<(), crate::DeviceError> {
        todo!()
    }

    unsafe fn present(
        &self,
        surface: &<Self::A as crate::Api>::Surface,
        texture: <Self::A as crate::Api>::SurfaceTexture,
    ) -> Result<(), crate::SurfaceError> {
        todo!()
    }

    unsafe fn get_timestamp_period(&self) -> f32 {
        0.0
    }
}
