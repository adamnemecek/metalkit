
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
    },

};

// #[macro_use]
use super::*;
// use crate::foreign_obj_type;

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

foreign_obj_type! {
    type CType = MTKView;
    pub struct View;
    pub struct ViewRef;
}

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

    pub fn device(&self) -> Option<DeviceRef> {
        unsafe { msg_send![self, device] }
    }

    pub fn set_device(&self, new_value: Option<DeviceRef>) {
        unsafe { msg_send![self, setDevice: new_value] }
    }

    //get only
    pub fn current_drawable(&self) -> Option<CAMetalDrawable> {
        unsafe { msg_send![self, currentDrawable] }
    }

    pub fn framebuffer_only(&self) -> bool {
        unsafe { msg_send![self, framebufferOnly] }
    }

    pub fn set_framebuffer_only(&self, new_value: bool) {
        unsafe { msg_send![self, setFramebufferOnly: new_value] }
    }

    //@available(OSX 10.15, *)
    pub fn depth_stencil_attachment_texture_usage(&self) -> MTLTextureUsage {
        unsafe { msg_send![self, depthStencilAttachmentTextureUsage] }
    }

    pub fn set_depth_stencil_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        unsafe { msg_send![self, setDepthStencilAttachmentTextureUsage: new_value] }
    }

    //@available(OSX 10.15, *)
    pub fn multisample_color_attachment_texture_usage(&self) -> MTLTextureUsage {
        unsafe { msg_send![self, multisampleColorAttachmentTextureUsage] }
    }

    pub fn set_multisample_color_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        unsafe { msg_send![self, setMultisampleColorAttachmentTextureUsage: new_value] }
    }

    pub fn presents_with_transaction(&self) -> bool {
        unsafe { msg_send![self, presentsWithTransaction] }
    }

    pub fn set_presents_with_transaction(&self, new_value: bool) {
        unsafe { msg_send![self, setPresentsWithTransaction: new_value] }
    }

    pub fn color_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, colorPixelFormat] }
    }

    pub fn set_color_pixel_format(&self, new_value: MTLPixelFormat) {
        unsafe { msg_send![self, setColorPixelFormat: new_value] }
    }

    pub fn depth_stencil_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, depthStencilPixelFormat] }
    }

    pub fn set_depth_stencil_pixel_format(&self, new_value: MTLPixelFormat) {
        unsafe { msg_send![self, setDepthStencilPixelFormat: new_value] }
    }

    pub fn sample_count(&self) -> Int {
        unsafe { msg_send![self, sampleCount] }
    }

    pub fn set_sample_count(&self, new_value: Int) {
        unsafe { msg_send![self, setSampleCount: new_value] }
    }

    pub fn clear_color(&self) -> MTLClearColor {
        unsafe { msg_send![self, clearColor] }
    }

    pub fn set_clear_color(&self, new_value: MTLClearColor) {
        unsafe { msg_send![self, setClearColor: new_value] }
    }

    pub fn clear_depth(&self) -> Double {
        unsafe { msg_send![self, clearDepth] }
    }

    pub fn set_clear_depth(&self, new_value: Double) {
        unsafe { msg_send![self, setClearDepth: new_value] }
    }

    pub fn clear_stencil(&self) -> UInt32 {
        unsafe { msg_send![self, clearStencil] }
    }

    pub fn set_clear_stencil(&self, new_value: UInt32) {
        unsafe { msg_send![self, setClearStencil: new_value] }
    }

    //get only
    pub fn depth_stencil_texture(&self) -> Option<MTLTexture> {
        unsafe { msg_send![self, depthStencilTexture] }
    }

    //get only
    pub fn multisample_color_texture(&self) -> Option<MTLTexture> {
        unsafe { msg_send![self, multisampleColorTexture] }
    }

    pub fn release_drawables(&self) {
        unsafe { msg_send![self, releaseDrawables] }
    }

    //get only
    pub fn current_render_pass_descriptor(&self) -> Option<MTLRenderPassDescriptor> {
        unsafe { msg_send![self, currentRenderPassDescriptor] }
    }

    pub fn preferred_frames_per_second(&self) -> Int {
        unsafe { msg_send![self, preferredFramesPerSecond] }
    }

    pub fn set_preferred_frames_per_second(&self, new_value: Int) {
        unsafe { msg_send![self, setPreferredFramesPerSecond: new_value] }
    }

    pub fn enable_setneeds_display(&self) -> bool {
        unsafe { msg_send![self, enableSetneedsDisplay] }
    }

    pub fn set_enable_setneeds_display(&self, new_value: bool) {
        unsafe { msg_send![self, setEnableEetneedsDisplay: new_value] }
    }

    pub fn autoresize_drawable(&self) -> bool {
        unsafe { msg_send![self, autoresizeDrawable] }
    }

    pub fn set_autoresize_drawable(&self, new_value: bool) {
        unsafe { msg_send![self, setAutoresizeDrawable: new_value] }
    }

    pub fn drawable_size(&self) -> CGSize {
        unsafe { msg_send![self, drawableSize] }
    }

    pub fn set_drawable_size(&self, new_value: CGSize) {
        unsafe { msg_send![self, setDrawableSize: new_value] }
    }

    //get only
    //@available(OSX 10.15, *)
    pub fn preferred_drawable_size(&self) -> CGSize {
        unsafe { msg_send![self, preferredDrawableSize] }
     }

    //get only
    //@available(OSX 10.15, *)
    pub fn preferred_device(&self) -> Option<MTLDevice> {
        unsafe { msg_send![self, preferredDevice] }
    }

    pub fn is_paused(&self) -> bool {
        unsafe { msg_send![self, isPaused] }
    }

    pub fn set_is_paused(&self, new_value: bool) {
        unsafe { msg_send![self, setIsPaused: new_value] }
    }

    //@available(OSX 10.12, *)
    pub fn colorspace(&self) -> Option<CGColorSpace> {
        unsafe { msg_send![self, colorspace] }
    }

    pub fn set_colorspace(&self, new_value: Option<CGColorSpace>) {
        unsafe { msg_send![self, setColorspace: new_value] }
    }

    pub fn draw(&self) {
        unsafe { msg_send![self, draw] }
    }
}