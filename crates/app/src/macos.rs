//! macOS: ask before quitting via ⌘Q, the app menu or the Dock.
//!
//! Slint always adds an app menu with the standard "Quit" item. It calls
//! `-[NSApplication terminate:]`, which ends the process without a close
//! request, so the prompt for unsaved changes would be skipped. winit's
//! application delegate does not implement `applicationShouldTerminate:`;
//! this module adds it at runtime.

use objc2::ffi::class_addMethod;
use objc2::runtime::{AnyClass, AnyObject, Imp, Sel};
use objc2::sel;
use std::cell::RefCell;

/// `NSApplicationTerminateReply` values.
const TERMINATE_CANCEL: usize = 0;
const TERMINATE_NOW: usize = 1;

/// `- (NSApplicationTerminateReply)applicationShouldTerminate:(NSApplication *)sender`
type ShouldTerminate = unsafe extern "C-unwind" fn(*mut AnyObject, Sel, *mut AnyObject) -> usize;

thread_local! {
    static MAY_QUIT: RefCell<Option<Box<dyn Fn() -> bool>>> = const { RefCell::new(None) };
}

/// Lets `may_quit` decide about quit requests from macOS: `true` quits right
/// away, `false` keeps CashFlow running (it then asks and quits by itself).
/// Needs the winit event loop, i.e. call it after the first window exists.
pub(crate) fn intercept_quit(may_quit: impl Fn() -> bool + 'static) {
    MAY_QUIT.set(Some(Box::new(may_quit)));
    let Some(class) = AnyClass::get(c"WinitApplicationDelegate") else {
        tracing::warn!("macOS application delegate not found; ⌘Q quits without asking");
        return;
    };
    let method: ShouldTerminate = should_terminate;
    // SAFETY: `method` matches the type encoding "Q@:@" (NSUInteger return, self, _cmd, sender).
    let added = unsafe {
        class_addMethod(
            std::ptr::from_ref(class).cast_mut(),
            sel!(applicationShouldTerminate:),
            std::mem::transmute::<ShouldTerminate, Imp>(method),
            c"Q@:@".as_ptr(),
        )
    };
    if !added.as_bool() {
        tracing::warn!("applicationShouldTerminate: is already implemented; ⌘Q quits without asking");
    }
}

unsafe extern "C-unwind" fn should_terminate(_this: *mut AnyObject, _cmd: Sel, _sender: *mut AnyObject) -> usize {
    let quit = MAY_QUIT.with_borrow(|may_quit| may_quit.as_ref().is_none_or(|may_quit| may_quit()));
    if quit { TERMINATE_NOW } else { TERMINATE_CANCEL }
}
