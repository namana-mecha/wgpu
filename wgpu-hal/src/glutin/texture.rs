#[derive(Debug)]
pub struct Texture;
impl crate::DynTexture for Texture {}
impl crate::DynSurfaceTexture for Texture {}
impl core::borrow::Borrow<dyn crate::DynTexture> for Texture {
    fn borrow(&self) -> &dyn crate::DynTexture {
        self
    }
}

#[derive(Debug)]
pub struct TextureView;
impl crate::DynTextureView for TextureView {}
