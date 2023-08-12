#![deny(clippy::all)]
pub mod ohos;

#[macro_use]
extern crate napi_derive;

use i_slint_core::window::{ffi::WindowAdapterRcOpaque, WindowAdapter};


#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}


#[no_mangle]
pub unsafe extern "C" fn slint_windowrc_init(out: *mut WindowAdapterRcOpaque) {
  assert_eq!(
    core::mem::size_of::<Rc<dyn WindowAdapter>>(),
    core::mem::size_of::<WindowAdapterRcOpaque>()
  );
  let win = with_platform(|b| b.create_window_adapter()).unwrap();
  core::ptr::write(out as *mut Rc<dyn WindowAdapter>, win);
}
