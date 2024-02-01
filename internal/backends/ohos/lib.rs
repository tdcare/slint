#![deny(clippy::all)]

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::rc::Rc;
use calloop::channel::Channel;
use i_slint_core::platform::{Platform, PlatformError, SetPlatformError};
use i_slint_core::window::ffi::WindowAdapterRcOpaque;
use i_slint_core::window::WindowAdapter;
use i_slint_core::with_platform;

use chrono::NaiveDate;
use libc::c_int;





// pub mod platform;
pub mod display;

pub mod renderer {
    // pub mod sw;
    pub mod femtovg;
}

pub mod calloop_backend;

// #[cfg(target_os = "ohos")]
pub mod ohoswindowadapter;

// #[macro_use]
// extern crate napi_derive;

// use hilog_binding::hilog_debug;
// use napi::bindgen_prelude::*;
use i_slint_core::api::EventLoopError;
use i_slint_core::string::format;
use PlatformError::NoPlatform;
// use slint::private_unstable_api::create_window_adapter;
use crate::calloop_backend::Backend;
use crate::calloop_backend::input::{GLOBAL_PROXY, OHOS_EVENT_SENDER};
use crate::calloop_backend::ohos::{OH_NativeXComponent_MouseEvent, OH_NativeXComponent_TouchEvent, OH_NativeXComponent_TouchEventType, OH_NativeXComponent_TouchPoint, OHOS_Input_Event};


// 直接使用egl
#[no_mangle]
pub fn glution_egl(ohos_widows: *mut c_void, w:u32, h:u32,error:*mut c_char) -> i32 {
    match display::glutin::glution_egl(ohos_widows,w,h){
        Ok(r) => {return 0}
        Err(e) => {
            // 将修改后的Rust字符串转换回C风格字符串
            let error_c_string = CString::new(format!("{:?}",e)).expect("Failed to create CString");
            // 将修改后的C风格字符串的内容复制回原始的字符串指针
            unsafe {
                libc::strcpy(error, error_c_string.as_ptr());
            }

            return -1
        }
    }
}
// slint 事件代理
pub fn slint_event_proxy(input_event:OHOS_Input_Event)->(i32,CString){
    let mut errored=false;
    let mut message_c_string=CString::new("运行成功").expect("Failed to create CString");

        match OHOS_EVENT_SENDER.get(){
            None => {
                errored=true;
                message_c_string = CString::new(format!("事件发送失败,没有找到发送端")).expect("Failed to create CString");
            }
            Some(sender) => {
               match sender.lock(){
                   Ok(s)=>{
                       match s.send(input_event){
                           Ok(_)=>{
                               message_c_string = CString::new(format!("事件发送成功")).expect("Failed to create CString");
                           },
                           Err(e)=>{
                               errored=true;
                               message_c_string = CString::new(format!("事件发送失败{}",e)).expect("Failed to create CString");
                           }
                       }
                   },
                   Err(e)=>{
                       errored=true;
                       message_c_string = CString::new(format!("事件发送失败{}",e)).expect("Failed to create CString");
                   }
               }
            }
        }


    return if errored {
        (-1,message_c_string)
    } else {
        (0,message_c_string)
    }
}