pub struct Surface {}
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
        timeout: Option<core::time::Duration>,
        fence: &<Self::A as crate::Api>::Fence,
    ) -> Result<Option<crate::AcquiredSurfaceTexture<Self::A>>, crate::SurfaceError> {
        todo!()
    }

    unsafe fn discard_texture(&self, texture: <Self::A as crate::Api>::SurfaceTexture) {
        todo!()
    }
}
