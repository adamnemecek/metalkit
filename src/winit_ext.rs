extern crate objc;

use cocoa::{
    appkit::{NSView},
    base::id as cocoa_id,
    foundation::{NSRange},
};

use core_graphics::geometry::CGSize;
use objc::runtime::YES;
use metal::*;
use winit::platform::macos::WindowExtMacOS;
use std::mem;

use winit::{
    event::{
        Event, WindowEvent,
    },
    event_loop::ControlFlow
};

pub fn create_metal_layer(
    device: &metal::Device,
    view: cocoa_id,
    size: CGSize,
) -> CoreAnimationLayer {

    let layer = CoreAnimationLayer::new();
    layer.set_device(device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);

    unsafe {
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer.as_ref()));
    }

    layer.set_drawable_size(size);

    layer
}

pub fn create_winit_metal_layer(
    device: &metal::Device,
    window: &winit::window::Window,
) -> CoreAnimationLayer {

    let view = window.ns_view() as cocoa_id;
    let draw_size = window.inner_size();
    let size = CGSize::new(draw_size.width as f64, draw_size.height as f64);

    create_metal_layer(
        device,
        view,
        size
    )
}