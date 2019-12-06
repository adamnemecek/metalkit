
#[macro_use]
extern crate objc;

#[macro_use]
extern crate foreign_types;

#[macro_use]
extern crate metal;

// mod macros;

mod view;
mod viewdelegate;

pub use {
    view::*,
    viewdelegate::*
};

use std::os::raw::c_void;
use cocoa::foundation::NSUInteger;
use objc::runtime::{Object, NO, YES};

#[inline]
unsafe fn obj_clone<T: 'static>(p: *mut T) -> *mut T {
    msg_send![(p as *mut Object), retain]
}


#[inline]
unsafe fn obj_drop<T>(p: *mut T) {
    msg_send![(p as *mut Object), release]
}


fn nsstring_as_str(nsstr: &objc::runtime::Object) -> &str {
    let bytes = unsafe {
        let bytes: *const std::os::raw::c_char = msg_send![nsstr, UTF8String];
        bytes as *const u8
    };
    let len: NSUInteger = unsafe { msg_send![nsstr, length] };
    unsafe {
        let bytes = std::slice::from_raw_parts(bytes, len as usize);
        std::str::from_utf8(bytes).unwrap()
    }
}

fn nsstring_from_str(string: &str) -> *mut objc::runtime::Object {
    const UTF8_ENCODING: usize = 4;

    let cls = class!(NSString);
    let bytes = string.as_ptr() as *const c_void;
    unsafe {
        let obj: *mut objc::runtime::Object = msg_send![cls, alloc];
        let obj: *mut objc::runtime::Object = msg_send![
            obj,
            initWithBytes:bytes
            length:string.len()
            encoding:UTF8_ENCODING
        ];
        let _: *mut c_void = msg_send![obj, autorelease];
        obj
    }
}

#[macro_use]
mod macr {
    #[macro_export]

    macro_rules! foreign_obj_type {
        {type CType = $raw_ident:ident;
        pub struct $owned_ident:ident;
        pub struct $ref_ident:ident;
        type ParentType = $parent_ref:ident;
        } => {
            foreign_obj_type! {
                type CType = $raw_ident;
                pub struct $owned_ident;
                pub struct $ref_ident;
            }

            impl ::std::ops::Deref for $ref_ident {
                type Target = $parent_ref;

                fn deref(&self) -> &$parent_ref {
                    unsafe { &*(self as *const $ref_ident as *const $parent_ref)  }
                }
            }
        };
        {type CType = $raw_ident:ident;
        pub struct $owned_ident:ident;
        pub struct $ref_ident:ident;
        } => {
            foreign_type! {
                type CType = $raw_ident;
                fn drop = crate::obj_drop;
                fn clone = crate::obj_clone;
                pub struct $owned_ident;
                pub struct $ref_ident;
            }

            unsafe impl ::objc::Message for $raw_ident {
            }
            unsafe impl ::objc::Message for $ref_ident {
            }

            impl ::std::fmt::Debug for $ref_ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    unsafe {
                        let string: *mut ::objc::runtime::Object = msg_send![self, debugDescription];
                        write!(f, "{}", crate::nsstring_as_str(&*string))
                    }
                }
            }

            impl ::std::fmt::Debug for $owned_ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    ::std::ops::Deref::deref(self).fmt(f)
                }
            }
        };
    }
}