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
use slint::SharedString;




// pub mod platform;
pub mod display{
    pub mod egldisplay;
    pub mod glutin;
}
pub mod renderer {
    // pub mod sw;
    pub mod femtovg;
}

pub mod calloop_backend;

// #[cfg(target_os = "ohos")]
pub mod ohoswindowadapter;

#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use i_slint_core::api::EventLoopError;
use i_slint_core::string::format;
use PlatformError::NoPlatform;
use slint::private_unstable_api::create_window_adapter;
use crate::calloop_backend::Backend;
use crate::calloop_backend::input::{GLOBAL_PROXY, OHOS_EVENT_SENDER};
use crate::calloop_backend::ohos::{OH_NativeXComponent_MouseEvent, OH_NativeXComponent_TouchEvent, OH_NativeXComponent_TouchEventType, OH_NativeXComponent_TouchPoint, OHOS_Input_Event};

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b + 100
}

#[napi]
pub async fn async_plus_100(p: Promise<u32>) -> Result<u32> {
    let v = p.await?;
    Ok(v + 210)
}

#[no_mangle]
pub unsafe extern "C" fn slint_windowrc_init(out: *mut std::os::raw::c_void) {
    // assert_eq!(
    //   core::mem::size_of::<Rc<dyn WindowAdapter>>(),
    //   core::mem::size_of::<WindowAdapterRcOpaque>()
    // );
    let win = with_platform(
        || Err(i_slint_core::platform::PlatformError::NoPlatform),
        |b| b.create_window_adapter(),
    ).unwrap();
    core::ptr::write(out as *mut Rc<dyn WindowAdapter>, win);
}

#[no_mangle]
pub unsafe extern "C" fn slint_egl(context: *mut std::os::raw::c_void,
                                   surface: *mut std::os::raw::c_void,
                                   display: *mut std::os::raw::c_void,
                                   get_proc_address:unsafe extern "C" fn(std::ffi::CStr)->*const std::ffi::c_void

) {
    // GlutinFemtoVGRenderer::new(context, surface, display,get_proc_address).expect("TODO: panic message");
}

// slint::include_modules!();
slint::slint!(import { Booker } from "booker.slint";);
// slint::slint!{
//     export component Booker {
//         Text {
//             text: "hello world";
//             color: green;
//         }
//     }
// }

/// 初始化 Slint 由OHOS 的C++ 进行调用
#[no_mangle]
pub fn init_slint(ohos_widows: *mut c_void,w:u32,h:u32,message:*mut c_char)-> i32 {
    let mut errored=false;
    let mut message_c_string=CString::new("运行成功").expect("Failed to create CString");

    match   Backend::new(ohos_widows, w, h) {
        Ok(backend) => {
           match  i_slint_core::platform::set_platform(Box::new(backend)){
               Ok(paltform) => {
                   message_c_string = CString::new(format!("设置平台信息成功")).expect("Failed to create CString");

               }
               Err(e) => {
                   errored=true;
                   message_c_string = CString::new(format!("设置失败:{:?}", e)).expect("Failed to create CString");
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
                            message_c_string = CString::new(format!("运行成功")).expect("Failed to create CString");

                        }
                        Err(e) => {
                            errored=true;
                            message_c_string = CString::new(format!("运行失败:{:?}", e)).expect("Failed to create CString");
                        }
                    }
                }
                Err(e) => {
                    errored=true;
                    message_c_string = CString::new(format!("构建失败:{:?}", e)).expect("Failed to create CString");
                }
            }
        }
        Err(e) => {
            message_c_string = CString::new(format!("{:?}", e)).expect("Failed to create CString");
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

/// 将OHOS中的事件传递给Slint 由OHOS 的C++ 进行调用
///触摸事件
#[no_mangle]
pub unsafe fn slint_touch(touch_event:*mut OH_NativeXComponent_TouchEvent, message:*mut c_char) ->i32{
    let event=OHOS_Input_Event::TouchEvent(*touch_event);
    let (status,message_c_string)=slint_event_proxy(event);
    unsafe {
        libc::strcpy(message, message_c_string.as_ptr());
    }
    status
}

 fn slint_event_proxy(input_event:OHOS_Input_Event)->(i32,CString){
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