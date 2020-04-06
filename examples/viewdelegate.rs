

use metalkit::{
    View,
    MetalView
};

use core_graphics::{
    geometry::{CGSize, CGRect, CGPoint},
};

fn main() {

    let rect: CGRect = CGRect::new(&CGPoint::new(0.0, 0.0), &CGSize::new(200.0, 200.0));
    let device = metal::Device::system_default().unwrap();
    let v = MetalView::new(rect, Some(&device));

    println!("{}", v.framebuffer_only());
}