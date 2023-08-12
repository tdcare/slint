// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

use alloc::boxed::Box;
use alloc::rc::Rc;
use core::{cell::RefCell, convert::Infallible};
use slint::platform::software_renderer;
use std::borrow::Borrow;

pub struct OHOSPlatform {
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
        // Ok(())
        loop {
            slint::platform::update_timers_and_animations();
            println!("执行事件。。。。");

            self.window.draw_if_needed(|renderer| {
                println!("在这里将内容输出到操作系统的显示子系统");

            });
        }
    }
}

// #[derive(Default)]
// pub struct OHOSPlatform {
//     window: RefCell<Option<Rc<slint::platform::software_renderer::MinimalSoftwareWindow>>>,
// }
//
// impl slint::platform::Platform for OHOSPlatform {
//     fn create_window_adapter(
//         &self,
//     ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
//         let window = slint::platform::software_renderer::MinimalSoftwareWindow::new(
//             slint::platform::software_renderer::RepaintBufferType::ReusedBuffer,
//         );
//         self.window.replace(Some(window.clone()));
//         Ok(window)
//     }
//
//
//
//     fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
//          // Ok(())
//         loop {
//             slint::platform::update_timers_and_animations();
//             println!("执行事件。。。。");
//
//             if let Some(window) = self.window.borrow().clone() {
//                 window.draw_if_needed(|renderer| {
//                     println!("更新窗口");
//
//                     // renderer.render_by_line(&mut buffer_provider);
//                 });
//                 if window.has_active_animations() {
//                     continue;
//                 }
//             }
//             // TODO
//         }
//     }
// }
//
