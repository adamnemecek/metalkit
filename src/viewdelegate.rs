
use crate::MetalView;
use core_graphics::geometry::{CGSize};

pub struct MetalViewDelegate  {
    // fn drawable_size_will_change(view: MetalView, size: CGSize);
        
    // fn draw(in: MetalView);
}

/// Creates a Cocoa delegate to use e.g. with `NSWindow.setDelegate_`.
/// Adds instance variables and methods to the definition.
///
/// # Example with NSWindowDelegate
/// ``` no_run
/// #[macro_use] extern crate cocoa;
/// #[macro_use] extern crate objc;
///
/// use cocoa::appkit::NSWindow;
/// use cocoa::base::nil;
///
/// # unsafe fn main() {
/// let my_window: id = NSWindow::alloc(nil);
///
/// extern fn on_enter_fullscreen(this: &Object, _cmd: Sel, _notification: id) {
///     unsafe {
///         let window: id = *this.get_ivar("window");
///         window.setToolbar_(nil);
///     }
/// }
///
/// my_window.setDelegate_(delegate!("MyWindowDelegate", {
///     window: id = my_window, // Declare instance variable(s)
///     (onWindowWillEnterFullscreen:) => on_enter_fullscreen as extern fn(&Object, Sel, id) // Declare function(s)
/// }));
/// # }
/// ```
#[macro_export]
macro_rules! delegate {
    (
        $name:expr, {
            $( ($($sel:ident :)+) => $func:expr),*
        }
    ) => (
        delegate!($name, {
            ,
            $( ($($sel :)+) => $func),*
        })
    );

    (
        $name:expr, {
            $($var:ident : $var_type:ty = $value:expr),* ,
            $( ($($sel:ident :)+) => $func:expr),*
        }
    ) => ({
        let mut decl = objc::declare::ClassDecl::new($name, class!(NSObject)).unwrap();

        $(
            decl.add_ivar::<$var_type>(stringify!($var));
        )*

        $(
            decl.add_method(sel!($($sel :)+), $func);
        )*

        let cl = decl.register();
        let delegate: id = msg_send![cl, alloc];

        $(
            (*delegate).set_ivar(stringify!($var), $value);
        )*

        delegate
    });
}