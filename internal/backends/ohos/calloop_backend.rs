// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

use std::cell::RefCell;
use std::num::NonZeroU32;
use std::os::fd::{AsFd, BorrowedFd, RawFd};
use std::os::raw::c_void;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;

use calloop::EventLoop;
use i_slint_core::platform::{Platform, PlatformError};
use i_slint_core::platform::WindowAdapter;
use crate::calloop_backend::input::{GLOBAL_PROXY, OHOS_EVENT_SENDER};
use crate::calloop_backend::ohos::OHOS_Input_Event;

use crate::ohoswindowadapter::{OhosRenderer, OhosWindowAdapter};
use crate::renderer::femtovg::FemtoVGRendererAdapter;


pub mod input;
pub mod ohos;

#[derive(Clone)]
struct Proxy {
    loop_signal: Arc<Mutex<Option<calloop::LoopSignal>>>,
    quit_loop: Arc<AtomicBool>,
    user_event_channel: Arc<Mutex<calloop::channel::Sender<Box<dyn FnOnce() + Send>>>>,
}

impl Proxy {
    fn new(event_channel: calloop::channel::Sender<Box<dyn FnOnce() + Send>>) -> Self {
        Self {
            loop_signal: Arc::new(Mutex::new(None)),
            quit_loop: Arc::new(AtomicBool::new(false)),
            user_event_channel: Arc::new(Mutex::new(event_channel)),
        }
    }
}

impl i_slint_core::platform::EventLoopProxy for Proxy {
    fn quit_event_loop(&self) -> Result<(), i_slint_core::api::EventLoopError> {
        let signal = self.loop_signal.lock().unwrap();
        signal.as_ref().map_or_else(
            || Err(i_slint_core::api::EventLoopError::EventLoopTerminated),
            |signal| {
                self.quit_loop.store(true, std::sync::atomic::Ordering::Release);
                signal.wakeup();
                Ok(())
            },
        )
    }

    fn invoke_from_event_loop(
        &self,
        event: Box<dyn FnOnce() + Send>,
    ) -> Result<(), i_slint_core::api::EventLoopError> {
        let user_event_channel = self.user_event_channel.lock().unwrap();
        user_event_channel
            .send(event)
            .map_err(|_| i_slint_core::api::EventLoopError::EventLoopTerminated)
    }
}

pub struct Backend {
    ohos_windows: *mut c_void,
    width: u32,
    height: u32,
    window: RefCell<Option<Rc<OhosWindowAdapter>>>,
    user_event_receiver: RefCell<Option<calloop::channel::Channel<Box<dyn FnOnce() + Send>>>>,
    ohos_event_receiver:RefCell<Option<calloop::channel::Channel<OHOS_Input_Event>>>,
    proxy: Proxy,
    sel_clipboard: RefCell<Option<String>>,
    clipboard: RefCell<Option<String>>,
}

impl Backend {
    pub fn new(ohos_windows: *mut c_void, width: u32, height: u32,) -> Result<Self, PlatformError> {
        let (user_event_sender, user_event_receiver) = calloop::channel::channel();
        let (ohos_event_sender, ohos_event_receiver) = calloop::channel::channel::<OHOS_Input_Event>();
        // let (sender,receiver)=mpsc::sync_channel::<OHOS_Input_Event>(1);

        // std::thread::spawn(|| {
        //     OHOS_EVENT_SENDER.set(sender);//.map_err(|e| format!("保存到全局变量出错了")).expect("TODO: panic message");
        // });//.join().map_err(|e| format!("保存到全局变量出错了"))?;
        OHOS_EVENT_SENDER.get_or_init(||Mutex::from(ohos_event_sender));
       // GLOBAL_PROXY.get_or_init(Default::default);

        Ok(Backend {
            ohos_windows,
            width,
            height,
            window: Default::default(),
            user_event_receiver: RefCell::new(Some(user_event_receiver)),
            ohos_event_receiver:RefCell::new(Some(ohos_event_receiver)),
            proxy: Proxy::new(user_event_sender),
            sel_clipboard: Default::default(),
            clipboard: Default::default(),
        })
    }

}

impl Platform for Backend {
    fn create_window_adapter(
        &self,
    ) -> Result<
        Rc<dyn WindowAdapter>,
        PlatformError,
    > {

        let renderer=FemtoVGRendererAdapter::new(self.ohos_windows,self.width,self.height)?;
        // This could be per-screen, once we support multiple outputs
        let rotation =
            std::env::var("SLINT_KMS_ROTATION").map_or(Ok(Default::default()), |rot_str| {
                rot_str
                    .as_str()
                    .try_into()
                    .map_err(|e| format!("Failed to parse SLINT_KMS_ROTATION: {e}"))
            })?;
        let adapter = OhosWindowAdapter::new(renderer,rotation)?;
        // return Err(PlatformError::NoEventLoopProvider);

       *self.window.borrow_mut() = Some(adapter.clone());

        Ok(adapter)
    }

    fn run_event_loop(&self) -> Result<(), PlatformError> {
        // return Err(PlatformError::from("错误".to_string()));
        // let adapter = self.window.borrow().as_ref().unwrap().clone();
        let adapter = self.window.borrow().as_ref().ok_or_else(|| format!("Error windows adapter "))?.clone();

        let mut loop_data = LoopData::default();
        let mut event_loop: EventLoop<LoopData> =
            EventLoop::try_new().map_err(|e| format!("Error creating event loop"))?;


        //初始化输入事件处理
        let Some(ohos_event_receiver) = self.ohos_event_receiver.borrow_mut().take() else {
            return Err(
                format!("Re-entering the calloop event loop is currently not supported").into()
            );
        };
        let mouse_position_property =
            input::OHOSInputHandler::init(adapter.window(), &event_loop.handle(),ohos_event_receiver)?;



        let Some(user_event_receiver) = self.user_event_receiver.borrow_mut().take() else {
            return Err(
                format!("Re-entering the ohos event loop is currently not supported").into()
            );
        };
        let callbacks_to_invoke_per_iteration = Rc::new(RefCell::new(Vec::new()));


        event_loop
            .handle()
            .insert_source(user_event_receiver, {
                let callbacks_to_invoke_per_iteration = callbacks_to_invoke_per_iteration.clone();
                move |event, _, _| {
                    let calloop::channel::Event::Msg(callback) = event else { return };
                    // Remember the callbacks and invoke them after updating the animation tick
                    callbacks_to_invoke_per_iteration.borrow_mut().push(callback);
                }
            })
            .map_err(
                |e: calloop::InsertError<calloop::channel::Channel<Box<dyn FnOnce() + Send>>>| {
                    format!("Error registering user event channel source: {e}")
                },
            )?;

        let loop_signal = event_loop.get_signal();
        *self.proxy.loop_signal.lock().unwrap() = Some(loop_signal.clone());

        let quit_loop = self.proxy.quit_loop.clone();
        quit_loop.store(false, std::sync::atomic::Ordering::Release);

        while !quit_loop.load(std::sync::atomic::Ordering::Acquire) {
            i_slint_core::platform::update_timers_and_animations();

            // Only after updating the animation tick, invoke callbacks from invoke_from_event_loop(). They
            // might set animated properties, which requires an up-to-date start time.
            for callback in callbacks_to_invoke_per_iteration.take().into_iter() {
                callback();
            }
            // return Err(PlatformError::from("调试".to_string()));

            if let Some(adapter) = self.window.borrow().as_ref() {
                adapter.register_event_loop(event_loop.handle())?;
                adapter.clone().render_if_needed(mouse_position_property.as_ref())?;
            };
            // return Err(PlatformError::from("调试1".to_string()));

            // let next_timeout = if adapter.window().has_active_animations() {
            //     Some(std::time::Duration::from_millis(16))
            // } else {
            //     i_slint_core::platform::duration_until_next_timer_update()
            // };

            let next_timeout = i_slint_core::platform::duration_until_next_timer_update();

            event_loop
                .dispatch(next_timeout, &mut loop_data)
                .map_err(|e| format!("Error dispatch events: {e}"))?;
        }

        Ok(())
    }

    fn new_event_loop_proxy(&self) -> Option<Box<dyn i_slint_core::platform::EventLoopProxy>> {
        Some(Box::new(self.proxy.clone()))
    }


    fn clipboard_text(&self, clipboard: i_slint_core::platform::Clipboard) -> Option<String> {
        match clipboard {
            i_slint_core::platform::Clipboard::DefaultClipboard => self.clipboard.borrow().clone(),
            i_slint_core::platform::Clipboard::SelectionClipboard => {
                self.sel_clipboard.borrow().clone()
            }
            _ => None,
        }
    }
    fn set_clipboard_text(&self, text: &str, clipboard: i_slint_core::platform::Clipboard) {
        match clipboard {
            i_slint_core::platform::Clipboard::DefaultClipboard => {
                *self.clipboard.borrow_mut() = Some(text.into())
            }
            i_slint_core::platform::Clipboard::SelectionClipboard => {
                *self.sel_clipboard.borrow_mut() = Some(text.into())
            }
            _ => (),
        }
    }
}

#[derive(Default)]
pub struct LoopData {}
//
// struct Device {
//     // in the future, use this from libseat: device_id: i32,
//     fd: RawFd,
// }
//
// impl AsFd for Device {
//     fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
//         unsafe { BorrowedFd::borrow_raw(self.fd) }
//     }
// }

pub type EventLoopHandle<'a> = calloop::LoopHandle<'a, LoopData>;
