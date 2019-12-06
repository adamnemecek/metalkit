
#[macro_use]
extern crate objc;

#[macro_use]
extern crate foreign_types;

#[macro_use]
extern crate metal;

mod view;
mod viewdelegate;

pub use {
    view::*,
    viewdelegate::*
};