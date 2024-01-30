#![deny(clippy::all)]

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;

// use i_slint_core::platform::{Platform, PlatformError, SetPlatformError};
// use i_slint_core::window::ffi::WindowAdapterRcOpaque;
// use i_slint_core::window::WindowAdapter;
// use i_slint_core::with_platform;

// use chrono::NaiveDate;
use libc::c_int;
use slint::SharedString;


// #[macro_use]
// extern crate napi_derive;
use napi_derive_ohos::napi;

// use hilog_binding::hilog_debug;

// use napi::bindgen_prelude::*;
// use i_slint_core::api::EventLoopError;
// use i_slint_core::string::format;
// use PlatformError::NoPlatform;

use i_slint_backend_ohos::calloop_backend::Backend;
use i_slint_backend_ohos::calloop_backend::input::{GLOBAL_PROXY, OHOS_EVENT_SENDER};
use i_slint_backend_ohos::calloop_backend::ohos::{OH_NativeXComponent_MouseEvent, OH_NativeXComponent_TouchEvent, OH_NativeXComponent_TouchEventType, OH_NativeXComponent_TouchPoint, OHOS_Input_Event};
use hilog_binding::hilog_debug;
#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    hilog_debug!("hello world!");
    a + b + 100
}

// #[napi]
// pub async fn async_plus_100(p: Promise<u32>) -> Result<u32> {
//     let v = p.await?;
//     hilog_debug!(
//         "test",
//         LogOptions {
//           tag: Some("testTag"),
//           domain: None
//       }
//     );
//     Ok(v + 210)
// }


// slint::include_modules!();
// slint::slint!(import { Demo } from "demo.slint";);
slint::slint!(import { Booker } from "booker.slint";);


/// 初始化 Slint 由OHOS 的C++ 进行调用
#[no_mangle]
pub fn init_demo(ohos_widows: *mut c_void,w:u32,h:u32,message:*mut c_char)-> i32 {
    let mut errored=false;
    let mut message_c_string=CString::new(format!("Running ")).expect("Failed to create CString");

    match   Backend::new(ohos_widows, w, h) {
        Ok(backend) => {
           match  slint::platform::set_platform(Box::new(backend)){
               Ok(paltform) => {
                   message_c_string = CString::new(format!("Configed Platform ")).expect("Failed to create CString");
               }
               Err(e) => {
                   errored=true;
                   message_c_string = CString::new(format!("Configed Platform Fail")).expect("Failed to create CString");
               }
           }
            match Booker::new() {
                Ok(main_window) => {
                    // main_window.on_validate_date(|date: SharedString| {
                    //     NaiveDate::parse_from_str(date.as_str(), "%d.%m.%Y").is_ok()
                    // });
                    // main_window.on_compare_date(|date1: SharedString, date2: SharedString| {
                    //     let date1 = match NaiveDate::parse_from_str(date1.as_str(), "%d.%m.%Y") {
                    //         Err(_) => return false,
                    //         Ok(x) => x,
                    //     };
                    //     let date2 = match NaiveDate::parse_from_str(date2.as_str(), "%d.%m.%Y") {
                    //         Err(_) => return false,
                    //         Ok(x) => x,
                    //     };
                    //     date1 <= date2
                    // });

                    // let w=  main_window.window();//.unwrap()
                    // create_window_adapter();
                    match main_window.run() {
                        Ok(run) => {
                            errored=false;
                            message_c_string = CString::new(format!("Run")).expect("Failed to create CString");
                        }
                        Err(e) => {
                            errored=true;
                            message_c_string = CString::new(format!("Run Fail:{:?}",e)).expect("Failed to create CString");
                        }
                    }
                }
                Err(e) => {
                    errored=true;
                    message_c_string = CString::new(format!("Build Fail")).expect("Failed to create CString");
                }
            }
        }
        Err(e) => {
            message_c_string = CString::new(format!("Err")).expect("Failed to create CString");
        }
    }
    unsafe {
        libc::strcpy(message, message_c_string.as_ptr());
    }

    return if errored {
        -1
    } else {
        0
    }

    }

/// 将OHOS中的事件传递给Slint 由OHOS 的C++ 进行调用
///触摸事件
#[no_mangle]
pub unsafe fn slint_touch(touch_event:*mut OH_NativeXComponent_TouchEvent, message:*mut c_char) ->i32{
    let event=OHOS_Input_Event::TouchEvent(*touch_event);
    let (status,message_c_string)=i_slint_backend_ohos::slint_event_proxy(event);
    unsafe {
        libc::strcpy(message, message_c_string.as_ptr());
    }
    status
}

