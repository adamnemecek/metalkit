

use metalkit::{
    View,
    MetalView
};

use core_graphics::{
    geometry::{CGSize, CGRect},
};

fn main() {

    let rect: CGRect = todo!();
    let device = metal::Device::system_default();
    let v = MetalView::new(rect, Some(&device));
    println!("hello world");
}