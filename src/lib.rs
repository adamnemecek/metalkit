
#[macro_use]
extern crate objc;

#[macro_use]
extern crate foreign_types;

#[macro_use]
extern crate metal;

mod macros;

mod view;
mod viewdelegate;

pub use {
    view::*,
    viewdelegate::*
};