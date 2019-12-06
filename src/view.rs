
use {
    // cocoa::foundation::NSUInteger,
    core_graphics::{
        geometry::CGSize,
        color_space::CGColorSpace,
    },
    metal::{
        CAMetalDrawable,
        // Double,
        // Int,
        MTLClearColor,
        MTLDevice,
        MTLPixelFormat,
        MTLRenderPassDescriptor,
        MTLTexture,
        MTLTextureUsage,
        DeviceRef
        // UInt32,
    }
};

// pub struct MTLTexture { }
// pub struct MTLRenderPassDescriptor { }
// pub struct MTLDevice { }
// pub struct CGColorSpace { }
// pub struct CAMetalDrawable { }
// pub struct MTLTextureUsage { }
// pub struct MTLPixelFormat { }
// pub struct MTLClearColor { }
pub struct UInt32 { }
pub struct Int { }
pub struct Double { }
// pub struct CGSize { }

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
    // 
    //

    // weak open var delegate: MTKViewDelegate?

    pub fn device(&self) -> Option<DeviceRef> {
        //impls: unsafe { msg_send![self, device] }
        unimplemented!()
    }

    pub fn set_device(&self, new_value: Option<DeviceRef>) {
        //impls: unsafe { msg_send![self, setDevice] }
        unimplemented!()
    }

    //get only
    pub fn current_drawable(&self) -> Option<CAMetalDrawable> {
        //impls: unsafe { msg_send![self, currentDrawable] }
        unimplemented!()
    }

    pub fn framebuffer_only(&self) -> bool {
        //impls: unsafe { msg_send![self, framebufferOnly] }
        unimplemented!()
    }

    pub fn set_framebuffer_only(&self, new_value: bool) {
        //impls: unsafe { msg_send![self, setFramebufferOnly] }
        unimplemented!()
    }

    //@available(OSX 10.15, *)
    pub fn depth_stencil_attachment_texture_usage(&self) -> MTLTextureUsage {
        //impls: unsafe { msg_send![self, depthStencilAttachmentTextureUsage] }
        unimplemented!()
    }

    pub fn set_depth_stencil_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        //impls: unsafe { msg_send![self, setDepthStencilAttachmentTextureUsage] }
        unimplemented!()
    }

    //@available(OSX 10.15, *)
    pub fn multisample_color_attachment_texture_usage(&self) -> MTLTextureUsage {
        //impls: unsafe { msg_send![self, multisampleColorAttachmentTextureUsage] }
        unimplemented!()
    }

    pub fn set_multisample_color_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        //impls: unsafe { msg_send![self, setMultisampleColorAttachmentTextureUsage] }
        unimplemented!()
    }

    pub fn presents_with_transaction(&self) -> bool {
        //impls: unsafe { msg_send![self, presentsWithTransaction] }
        unimplemented!()
    }

    pub fn set_presents_with_transaction(&self, new_value: bool) {
        //impls: unsafe { msg_send![self, setPresentsWithTransaction] }
        unimplemented!()
    }

    pub fn color_pixel_format(&self) -> MTLPixelFormat {
        //impls: unsafe { msg_send![self, colorPixelFormat] }
        unimplemented!()
    }

    pub fn set_color_pixel_format(&self, new_value: MTLPixelFormat) {
        //impls: unsafe { msg_send![self, setColorPixelFormat] }
        unimplemented!()
    }

    pub fn depth_stencil_pixel_format(&self) -> MTLPixelFormat {
        //impls: unsafe { msg_send![self, depthStencilPixelFormat] }
        unimplemented!()
    }

    pub fn set_depth_stencil_pixel_format(&self, new_value: MTLPixelFormat) {
        //impls: unsafe { msg_send![self, setDepthStencilPixelFormat] }
        unimplemented!()
    }

    pub fn sample_count(&self) -> Int {
        //impls: unsafe { msg_send![self, sampleCount] }
        unimplemented!()
    }

    pub fn set_sample_count(&self, new_value: Int) {
        //impls: unsafe { msg_send![self, setSampleCount] }
        unimplemented!()
    }

    pub fn clear_color(&self) -> MTLClearColor {
        //impls: unsafe { msg_send![self, clearColor] }
        unimplemented!()
    }

    pub fn set_clear_color(&self, new_value: MTLClearColor) {
        //impls: unsafe { msg_send![self, setClearColor] }
        unimplemented!()
    }

    pub fn clear_depth(&self) -> Double {
        //impls: unsafe { msg_send![self, clearDepth] }
        unimplemented!()
    }

    pub fn set_clear_depth(&self, new_value: Double) {
        //impls: unsafe { msg_send![self, setClearDepth] }
        unimplemented!()
    }

    pub fn clear_stencil(&self) -> UInt32 {
        //impls: unsafe { msg_send![self, clearStencil] }
        unimplemented!()
    }

    pub fn set_clear_stencil(&self, new_value: UInt32) {
        //impls: unsafe { msg_send![self, setClearStencil] }
        unimplemented!()
    }

    //get only
    pub fn depth_stencil_texture(&self) -> Option<MTLTexture> {
        //impls: unsafe { msg_send![self, depthStencilTexture] }
        unimplemented!()
    }

    //get only
    pub fn multisample_color_texture(&self) -> Option<MTLTexture> {
        //impls: unsafe { msg_send![self, multisampleColorTexture] }
        unimplemented!()
    }

    pub fn release_drawables(&self) {
        //impls: unsafe { msg_send![self, releaseDrawables] }
        unimplemented!()
    }

    //get only
    pub fn current_render_pass_descriptor(&self) -> Option<MTLRenderPassDescriptor> {
        //impls: unsafe { msg_send![self, currentRenderPassDescriptor] }
        unimplemented!()
    }

    pub fn preferred_frames_per_second(&self) -> Int {
        //impls: unsafe { msg_send![self, preferredFramesPerSecond] }
        unimplemented!()
    }

    pub fn set_preferred_frames_per_second(&self, new_value: Int) {
        //impls: unsafe { msg_send![self, setPreferredFramesPerSecond] }
        unimplemented!()
    }

    pub fn enable_setneeds_display(&self) -> bool {
        //impls: unsafe { msg_send![self, enableSetneedsDisplay] }
        unimplemented!()
    }

    pub fn set_enable_setneeds_display(&self, new_value: bool) {
        //impls: unsafe { msg_send![self, setEnableEetneedsDisplay] }
        unimplemented!()
    }

    pub fn autoresize_drawable(&self) -> bool {
        //impls: unsafe { msg_send![self, autoresizeDrawable] }
        unimplemented!()
    }

    pub fn set_autoresize_drawable(&self, new_value: bool) {
        //impls: unsafe { msg_send![self, setAutoresizeDrawable] }
        unimplemented!()
    }

    pub fn drawable_size(&self) -> CGSize {
        //impls: unsafe { msg_send![self, drawableSize] }
        unimplemented!()
    }

    pub fn set_drawable_size(&self, size: CGSize) {
        //impls: unsafe { msg_send![self, setDrawableSize] }
        unimplemented!()
    }

    //get only
    //@available(OSX 10.15, *)
    pub fn preferred_drawable_size(&self) -> CGSize {
        //impls: unsafe { msg_send![self, preferredDrawableSize] }
        unimplemented!()
     }

    //get only
    //@available(OSX 10.15, *)
    pub fn preferred_device(&self) -> Option<MTLDevice> {
        //impls: unsafe { msg_send![self, preferredDevice] }
        unimplemented!()
    }

    pub fn is_paused(&self) -> bool {
        //impls: unsafe { msg_send![self, isPaused] }
        unimplemented!()
    }

    pub fn set_is_paused(&self, new_value: bool) {
        //impls: unsafe { msg_send![self, setIsPaused] }
        unimplemented!()
    }

    //@available(OSX 10.12, *)
    pub fn colorspace(&self) -> Option<CGColorSpace> {
        //impls: unsafe { msg_send![self, colorspace] }
        unimplemented!()
    }

    pub fn set_colorspace(&self, new_value: Option<CGColorSpace>) {
        //impls: unsafe { msg_send![self, setColorspace] }
        unimplemented!()
    }

    pub fn draw(&self) {
        //impls: unsafe { msg_send![self, draw] }
        unimplemented!()
    }
}