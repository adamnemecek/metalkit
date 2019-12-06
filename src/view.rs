
use cocoa::foundation::NSUInteger;
// use metal::foreign_obj_type;
use metal::{
    CAMetalDrawable,
    // CGColorSpace,
    // CGSize,
    // Double,
    // Int,
    MTLClearColor,
    MTLDevice,
    MTLPixelFormat,
    MTLRenderPassDescriptor,
    MTLTexture,
    MTLTextureUsage,
    // UInt32,
};

// pub struct MTLTexture { }
// pub struct MTLRenderPassDescriptor { }
// pub struct MTLDevice { }
pub struct CGColorSpace { }
// pub struct CAMetalDrawable { }
// pub struct MTLTextureUsage { }
// pub struct MTLPixelFormat { }
// pub struct MTLClearColor { }
pub struct UInt32 { }
pub struct Int { }
pub struct Double { }
pub struct CGSize { }

pub enum MTKView {}

// foreign_obj_type! {
    // type CType = MTKView;
    pub struct View;
    pub struct ViewRef;
// }

impl View {
    pub fn new<'a>() -> &'a ViewRef {
        unsafe {
            let class = class!(MTKView);
            msg_send![class, new]
        }
    }
}

impl ViewRef {

    // weak open var delegate: MTKViewDelegate?

    pub fn device(&self) -> Option<MTLDevice> {
        unimplemented!()
    }

    pub fn set_device(&self, new_value: Option<MTLDevice>) {
        unimplemented!()
    }

    //get only
    pub fn currentdrawable(&self) -> Option<CAMetalDrawable> {
        unimplemented!()
    }

    pub fn framebuffer_only(&self) -> bool {
        unimplemented!()
    }

    pub fn set_framebuffer_only(&self, new_value: bool) {
        unimplemented!()
    }

    //@available(OSX 10.15, *)
    pub fn depth_stencil_attachment_texture_usage(&self) -> MTLTextureUsage {
        unimplemented!()
    }

    pub fn set_depth_stencil_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        unimplemented!()
    }

    //@available(OSX 10.15, *)
    pub fn multisample_color_attachment_texture_usage(&self) -> MTLTextureUsage {
        unimplemented!()
    }

    pub fn set_multisample_color_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        unimplemented!()
    }

    pub fn presents_with_transaction(&self) -> bool {
        unimplemented!()
    }

    pub fn set_presents_with_transaction(&self, new_value: bool){
        unimplemented!()
    }

    pub fn color_pixel_format(&self) -> MTLPixelFormat {
        unimplemented!()
    }

    pub fn set_color_pixel_format(&self, new_value: MTLPixelFormat) {
        unimplemented!()
    }

    pub fn depth_stencil_pixel_format(&self) -> MTLPixelFormat {
        unimplemented!()
    }

    pub fn set_depth_stencil_pixel_format(&self, new_value: MTLPixelFormat) {
        unimplemented!()
    }

    pub fn sample_count(&self) -> Int {
        unimplemented!()
    }

    pub fn set_sample_count(&self, new_value: Int) {
        unimplemented!()
    }

    pub fn clear_color(&self) -> MTLClearColor {
        unimplemented!()
    }

    pub fn set_clear_color(&self, new_value: MTLClearColor) {
        unimplemented!()
    }

    pub fn clear_depth(&self) -> Double {
        unimplemented!()
    }

    pub fn set_clear_depth(&self, new_value: Double) {
        unimplemented!()
    }

    pub fn clear_stencil(&self) -> UInt32 {
        unimplemented!()
    }

    pub fn set_clear_stencil(&self, new_value: UInt32) {
        unimplemented!()
    }

    //get only
    pub fn depth_stencil_texture(&self) -> Option<MTLTexture> {
        unimplemented!()
    }

    //get only
    pub fn multisample_color_texture(&self) -> Option<MTLTexture> {
        unimplemented!()
    }

    pub fn release_drawables(&self) {
        unimplemented!()
    }

    //get only
    pub fn current_render_pass_descriptor(&self) -> Option<MTLRenderPassDescriptor> {
        unimplemented!()
    }

    pub fn preferred_frames_per_second(&self) -> Int {
        unimplemented!()
    }

    pub fn set_preferred_frames_per_second(&self, new_value: Int) {
        unimplemented!()
    }

    pub fn enable_setneeds_display(&self) -> bool {
        unimplemented!()
    }

    pub fn set_enable_setneeds_display(&self, new_value: bool) {
        unimplemented!()
    }

    pub fn autoresize_drawable(&self) -> bool {
        unimplemented!()
    }

    pub fn set_autoresize_drawable(&self, new_value: bool) {
        unimplemented!()
    }

    pub fn drawable_size(&self) -> CGSize {
        unimplemented!()
    }

    pub fn set_drawable_size(&self, size: CGSize) {
        unimplemented!()
    }

    //get only
    //@available(OSX 10.15, *)
    pub fn preferred_drawable_size(&self) -> CGSize {
        unimplemented!()
     }

    //get only
    //@available(OSX 10.15, *)
    pub fn preferred_device(&self) -> Option<MTLDevice> {
        unimplemented!()
    }

    pub fn is_paused(&self) -> bool {
        unimplemented!()
    }

    pub fn set_is_paused(&self, new_value: bool) {
        unimplemented!()
    }

    //@available(OSX 10.12, *)
    pub fn colorspace(&self) -> Option<CGColorSpace> {
        unimplemented!()
    }

    pub fn set_colorspace(&self, new_value: Option<CGColorSpace>) {
        unimplemented!()
    }

    pub fn draw(&self) {
        unimplemented!()
    }
}