// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

extern crate alloc;

use alloc::{
    boxed::Box,
    format,
    rc::Rc,
    string::{String, ToString},
};
use core::{slice, time::Duration};
use slint::{platform::software_renderer, SharedString};

slint::include_modules!();


struct OHOSPlatform {
    window: Rc<software_renderer::MinimalSoftwareWindow>,
}

impl Default for OHOSPlatform {
    fn default() -> Self {
        Self {
            window: software_renderer::MinimalSoftwareWindow::new(
                software_renderer::RepaintBufferType::ReusedBuffer,
            ),
        }
    }
}

impl slint::platform::Platform for OHOSPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }

    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        Ok(())
    }
}

fn main()  {
    slint::platform::set_platform(Box::<OHOSPlatform>::default()).unwrap();
    unsafe {
        let fontconfig = libloading::Library::new("/system/lib/libwm.z.so");
        // if fontconfig.is_ok() {
        //     println!("加载成功");
        // }else {
        //     println!("加载失败！");
        // }
        match fontconfig {
            Ok(f)=>{
                println!("加载成功");
            }
            Err(e)=>{
                println!("加载失败！{}",e);
            }
        }
    }

    println!("Slint执行成功！");
}
