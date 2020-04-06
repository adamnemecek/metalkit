
use {
    // cocoa::foundation::NSUInteger,
    core_graphics::{
        geometry::{CGSize, CGRect},
        color_space::CGColorSpace,
    },
    cocoa::{
        // NSView
    },
    metal::{
        CoreAnimationDrawableRef,
        MTLClearColor,
        MTLDevice,
        MTLPixelFormat,
        // MTLRenderPassDescriptor,
        RenderPassDescriptorRef,
        // MTLTexture,
        MTLTextureUsage,
        DeviceRef,
        TextureRef,
    },

};

// #[macro_use]
use super::*;

pub struct UInt32 { }
pub struct Int { }
pub struct Double { }

pub enum NSView {}
foreign_obj_type! {
    type CType = NSView;
    pub struct View;
    pub struct ViewRef;
}

pub enum MTKView {}

foreign_obj_type! {
    type CType = MTKView;
    pub struct MetalView;
    pub struct MetalViewRef;
    type ParentType = ViewRef;
}

impl MetalView {
    pub fn new<'a>(frame: CGRect, device: Option<&DeviceRef>) -> &'a MetalViewRef {
        unsafe {
            let class = class!(MTKView);
            msg_send![class, new]
        }
    }
}

impl MetalViewRef {

    // weak open var delegate: MTKViewDelegate?
    pub fn delegate(&self) -> Option<MetalViewDelegate> {
        unsafe { msg_send![self, delegate] }
    }

    pub fn set_delegate(&self, new_value: Option<MetalViewDelegate>) {
        unsafe { msg_send![self, setDelegate: new_value] }
    }

    pub fn device(&self) -> Option<&DeviceRef> {
        unsafe { msg_send![self, device] }
    }

    pub fn set_device(&self, new_value: Option<&DeviceRef>) {
        unsafe { msg_send![self, setDevice: new_value] }
    }

    pub fn preferred_device(&self) -> Option<&DeviceRef> {
        unsafe { msg_send![self, preferredDevice] }
    }

    pub fn color_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, colorPixelFormat] }
    }

    pub fn set_color_pixel_format(&self, new_value: MTLPixelFormat) {
        unsafe { msg_send![self, setColorPixelFormat: new_value] }
    }

    //@available(OSX 10.12, *)
    pub fn colorspace(&self) -> Option<CGColorSpace> {
        unsafe { msg_send![self, colorspace] }
    }

    pub fn set_colorspace(&self, new_value: Option<CGColorSpace>) {
        unsafe { msg_send![self, setColorspace: new_value] }
    }

    pub fn framebuffer_only(&self) -> bool {
        unsafe { msg_send![self, framebufferOnly] }
    }

    pub fn set_framebuffer_only(&self, new_value: bool) {
        unsafe { msg_send![self, setFramebufferOnly: new_value] }
    }

    pub fn drawable_size(&self) -> CGSize {
        unsafe { msg_send![self, drawableSize] }
    }

    pub fn set_drawable_size(&self, new_value: CGSize) {
        unsafe { msg_send![self, setDrawableSize: new_value] }
    }

    pub fn preferred_drawable_size(&self) -> CGSize {
        unsafe { msg_send![self, preferredDrawableSize] }
     }

    pub fn autoresize_drawable(&self) -> bool {
        unsafe { msg_send![self, autoresizeDrawable] }
    }

    pub fn set_autoresize_drawable(&self, new_value: bool) {
        unsafe { msg_send![self, setAutoresizeDrawable: new_value] }
    }

    pub fn clear_color(&self) -> MTLClearColor {
        unsafe { msg_send![self, clearColor] }
    }

    pub fn set_clear_color(&self, new_value: MTLClearColor) {
        unsafe { msg_send![self, setClearColor: new_value] }
    }

    pub fn depth_stencil_pixel_format(&self) -> MTLPixelFormat {
        unsafe { msg_send![self, depthStencilPixelFormat] }
    }

    pub fn set_depth_stencil_pixel_format(&self, new_value: MTLPixelFormat) {
        unsafe { msg_send![self, setDepthStencilPixelFormat: new_value] }
    }

    pub fn depth_stencil_attachment_texture_usage(&self) -> MTLTextureUsage {
        unsafe { msg_send![self, depthStencilAttachmentTextureUsage] }
    }

    pub fn set_depth_stencil_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        unsafe { msg_send![self, setDepthStencilAttachmentTextureUsage: new_value] }
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

    pub fn sample_count(&self) -> Int {
        unsafe { msg_send![self, sampleCount] }
    }

    pub fn set_sample_count(&self, new_value: Int) {
        unsafe { msg_send![self, setSampleCount: new_value] }
    }

    pub fn multisample_color_attachment_texture_usage(&self) -> MTLTextureUsage {
        unsafe { msg_send![self, multisampleColorAttachmentTextureUsage] }
    }

    pub fn set_multisample_color_attachment_texture_usage(&self, new_value: MTLTextureUsage) {
        unsafe { msg_send![self, setMultisampleColorAttachmentTextureUsage: new_value] }
    }

    pub fn current_render_pass_descriptor(&self) -> Option<&RenderPassDescriptorRef> {
        unsafe { msg_send![self, currentRenderPassDescriptor] }
    }

    pub fn current_drawable(&self) -> Option<&CoreAnimationDrawableRef> {
        unsafe { msg_send![self, currentDrawable] }
    }

    pub fn depth_stencil_texture(&self) -> Option<&TextureRef> {
        unsafe { msg_send![self, depthStencilTexture] }
    }

    pub fn multisample_color_texture(&self) -> Option<&TextureRef> {
        unsafe { msg_send![self, multisampleColorTexture] }
    }

   pub fn preferred_frames_per_second(&self) -> Int {
        unsafe { msg_send![self, preferredFramesPerSecond] }
    }

    pub fn set_preferred_frames_per_second(&self, new_value: Int) {
        unsafe { msg_send![self, setPreferredFramesPerSecond: new_value] }
    }

    pub fn is_paused(&self) -> bool {
        unsafe { msg_send![self, isPaused] }
    }

    pub fn set_is_paused(&self, new_value: bool) {
        unsafe { msg_send![self, setIsPaused: new_value] }
    }

    pub fn enable_setneeds_display(&self) -> bool {
        unsafe { msg_send![self, enableSetneedsDisplay] }
    }

    pub fn set_enable_setneeds_display(&self, new_value: bool) {
        unsafe { msg_send![self, setEnableEetneedsDisplay: new_value] }
    }

    pub fn draw(&self) {
        unsafe { msg_send![self, draw] }
    }

    pub fn presents_with_transaction(&self) -> bool {
        unsafe { msg_send![self, presentsWithTransaction] }
    }

    pub fn set_presents_with_transaction(&self, new_value: bool) {
        unsafe { msg_send![self, setPresentsWithTransaction: new_value] }
    }

    pub fn release_drawables(&self) {
        unsafe { msg_send![self, releaseDrawables] }
    }
}