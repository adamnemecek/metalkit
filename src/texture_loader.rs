
use super::*;
use metal::{
    DeviceRef,
    Texture
};
pub enum MTLTextureLoader {}


foreign_obj_type! {
    type CType = MTLTextureLoader;
    pub struct TextureLoader;
    pub struct TextureLoaderRef;
}

impl TextureLoader {
    pub fn new<'a>(device: Option<&DeviceRef>) -> &'a TextureLoaderRef {
        unsafe {
            let class = class!(MTLTextureLoader);
            msg_send![class, new]
        }
    }
}

impl TextureLoaderRef {
    pub fn device(&self) -> Option<&DeviceRef> {
        unsafe { msg_send![self, device] }
    }

    // pub fn new_texture(
    //     &self,
    //     URL: URL,
    //     options: [MTKTextureLoader.Option : Any]?) -> Texture {
    // todo!()
    // }

    // pub fn new_texture_async(
    //     &self,
    //     URL: URL,
    //     options: [MTKTextureLoader.Option : Any]?,
    //     completionHandler: MTKTextureLoader.Callback) {
    // todo!()
    // }

    // pub fn new_textures(
    //     &self,
    //     URLs: [URL],
    //     options: [MTKTextureLoader.Option : Any]?,
    //     error: NSErrorPointer
    // ) -> Vec<Texture> {
    // todo!()
    // }

    // pub fn new_textures_async(
    //     &self,
    //     URLs: [URL],
    //     options: [MTKTextureLoader.Option : Any]?,
    //     completionHandler: MTKTextureLoader.ArrayCallback) {
    // todo!()
    // }
}