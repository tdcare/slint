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
    hilog_debug!(
        "tdcare",
        LogOptions {
          tag: Some("testTag"),
          domain: None
      }
    );
    a + b + 200
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
// slint::slint!(import { MainWindow } from "memory.slint";);
// slint::slint!(import { Booker } from "booker.slint";);



/// 初始化 Slint 由OHOS 的C++ 进行调用
#[no_mangle]
pub fn init_demo(ohos_widows: *mut c_void,w:u32,h:u32,message:*mut c_char)-> i32 {
    slint::slint!(import { Demo } from "demo.slint";);

    let mut errored=false;
    let mut message_c_string=CString::new(format!("Running ")).expect("Failed to create CString");
    hilog_debug!("hello world1");

    let p=Backend::new(ohos_widows, w, h).unwrap();
    slint::platform::set_platform(Box::new(p)).unwrap();
    let demo=Demo::new().unwrap();
      demo.run().unwrap();

    // match   Backend::new(ohos_widows, w, h) {
    //     Ok(backend) => {
    //        match  slint::platform::set_platform(Box::new(backend)){
    //            Ok(paltform) => {
    //                message_c_string = CString::new(format!("Configed Platform ")).expect("Failed to create CString");
    //            }
    //            Err(e) => {
    //                errored=true;
    //                message_c_string = CString::new(format!("Configed Platform Fail")).expect("Failed to create CString");
    //            }
    //        }
    //         match Demo::new() {
    //             Ok(main_window) => {
    //                 match main_window.run() {
    //                     Ok(run) => {
    //                         errored=false;
    //                         message_c_string = CString::new(format!("Run")).expect("Failed to create CString");
    //                     }
    //                     Err(e) => {
    //                         errored=true;
    //                         message_c_string = CString::new(format!("Run Fail:{:?}",e)).expect("Failed to create CString");
    //                     }
    //                 }
    //             }
    //             Err(e) => {
    //                 errored=true;
    //                 message_c_string = CString::new(format!("Build Fail")).expect("Failed to create CString");
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         message_c_string = CString::new(format!("Err")).expect("Failed to create CString");
    //     }
    // }
    //

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
pub fn init_crud(ohos_widows: *mut c_void,w:u32,h:u32,message:*mut c_char)-> i32 {
    use slint::{Model, ModelExt, SharedString, StandardListViewItem, VecModel};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct Name {
        first: String,
        last: String,
    }


    slint::slint!(import { MainWindow } from "crud.slint";);

    let mut errored=false;
    let mut message_c_string=CString::new(format!("Running ")).expect("Failed to create CString");


    let p=Backend::new(ohos_widows, w, h).unwrap();
    slint::platform::set_platform(Box::new(p)).unwrap();

    let main_window = MainWindow::new().unwrap();

    let prefix = Rc::new(RefCell::new(SharedString::from("")));
    let prefix_for_wrapper = prefix.clone();

    let model = Rc::new(VecModel::from(vec![
        Name { first: "Hans".to_string(), last: "Emil".to_string() },
        Name { first: "Max".to_string(), last: "Mustermann".to_string() },
        Name { first: "Roman".to_string(), last: "Tisch".to_string() },
    ]));

    let filtered_model = Rc::new(
        model
            .clone()
            .map(|n| StandardListViewItem::from(slint::format!("{}, {}", n.last, n.first)))
            .filter(move |e| e.text.starts_with(prefix_for_wrapper.borrow().as_str())),
    );

    main_window.set_names_list(filtered_model.clone().into());

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        main_window.on_createClicked(move || {
            let main_window = main_window_weak.unwrap();
            let new_entry = Name {
                first: main_window.get_name().to_string(),
                last: main_window.get_surname().to_string(),
            };
            model.push(new_entry);
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        main_window.on_updateClicked(move || {
            let main_window = main_window_weak.unwrap();

            let updated_entry = Name {
                first: main_window.get_name().to_string(),
                last: main_window.get_surname().to_string(),
            };

            let row = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
            model.set_row_data(row, updated_entry);
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let model = model.clone();
        let filtered_model = filtered_model.clone();
        main_window.on_deleteClicked(move || {
            let main_window = main_window_weak.unwrap();

            let index = filtered_model.unfiltered_row(main_window.get_current_item() as usize);
            model.remove(index);
        });
    }

    {
        let main_window_weak = main_window.as_weak();
        let filtered_model = filtered_model.clone();
        main_window.on_prefixEdited(move || {
            let main_window = main_window_weak.unwrap();
            *prefix.borrow_mut() = main_window.get_prefix();
            filtered_model.reset();
        });
    }

    main_window.run().unwrap();


    unsafe {
        libc::strcpy(message, message_c_string.as_ptr());
    }

    return if errored {
        -1
    } else {
        0
    }
}