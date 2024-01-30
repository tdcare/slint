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

    // seat: Rc<RefCell<libseat::Seat>>,
    window: RefCell<Option<Rc<OhosWindowAdapter>>>,
    user_event_receiver: RefCell<Option<calloop::channel::Channel<Box<dyn FnOnce() + Send>>>>,
    ohos_event_receiver:RefCell<Option<calloop::channel::Channel<OHOS_Input_Event>>>,
    proxy: Proxy,
    // renderer_factory: for<'a> fn(
    //        *mut c_void,  u32, u32,
    // ) -> Result<
    //     Box<dyn OhosRenderer>,
    //     PlatformError,
    // >,
    sel_clipboard: RefCell<Option<String>>,
    clipboard: RefCell<Option<String>>,
}

impl Backend {
    pub fn new(ohos_windows: *mut c_void, width: u32, height: u32,) -> Result<Self, PlatformError> {
        let (user_event_sender, user_event_receiver) = calloop::channel::channel();
        let (ohos_event_sender, ohos_event_receiver) = calloop::channel::channel::<OHOS_Input_Event>();
        let (sender,receiver)=mpsc::sync_channel::<OHOS_Input_Event>(1);

        // std::thread::spawn(|| {
        //     OHOS_EVENT_SENDER.set(sender);//.map_err(|e| format!("保存到全局变量出错了")).expect("TODO: panic message");
        // });//.join().map_err(|e| format!("保存到全局变量出错了"))?;
        OHOS_EVENT_SENDER.get_or_init(||Mutex::from(ohos_event_sender));
       // GLOBAL_PROXY.get_or_init(Default::default);

        // let renderer_factory = crate::renderer::femtovg::FemtoVGRendererAdapter::new;
        // let renderer_factory = match renderer_name {
        //     #[cfg(feature = "renderer-skia-vulkan")]
        //     Some("skia-vulkan") => crate::renderer::skia::SkiaRendererAdapter::new_vulkan,
        //     #[cfg(feature = "renderer-skia-opengl")]
        //     Some("skia-opengl") => crate::renderer::skia::SkiaRendererAdapter::new_opengl,
        //     #[cfg(feature = "renderer-femtovg")]
        //     Some("femtovg") => crate::renderer::femtovg::FemtoVGRendererAdapter::new,
        //     None => crate::renderer::try_skia_then_femtovg,
        //     Some(renderer_name) => {
        //         eprintln!(
        //             "slint linuxkms backend: unrecognized renderer {}, falling back default",
        //             renderer_name
        //         );
        //         crate::renderer::try_skia_then_femtovg
        //     }
        // };

        // let seat_active = Rc::new(RefCell::new(false));
        //
        // //libseat::set_log_level(libseat::LogLevel::Debug);
        //
        // let mut seat = {
        //     let seat_active = seat_active.clone();
        //     libseat::Seat::open(
        //         move |_seat, event| match event {
        //             libseat::SeatEvent::Enable => {
        //                 *seat_active.borrow_mut() = true;
        //             }
        //             libseat::SeatEvent::Disable => {
        //                 unimplemented!("Seat deactivation is not implemented");
        //             }
        //         },
        //         None,
        //     )
        //         .map_err(|e| format!("Error opening session with libseat: {e}"))?
        // };
        //
        // while !(*seat_active.borrow()) {
        //     if seat.dispatch(5000).map_err(|e| format!("Error waiting for seat activation: {e}"))?
        //         == 0
        //     {
        //         return Err(format!("Timeout while waiting to activate session").into());
        //     }
        // }

        Ok(Backend {
            ohos_windows,
            width,
            height,
            // seat: Rc::new(RefCell::new(seat)),
            window: Default::default(),

            user_event_receiver: RefCell::new(Some(user_event_receiver)),

            ohos_event_receiver:RefCell::new(Some(ohos_event_receiver)),

            proxy: Proxy::new(user_event_sender),
            // renderer_factory,
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
        // let renderer = (self.renderer_factory)(&|device: &std::path::Path| {
        //     let (_, fd) = self
        //         .seat
        //         .borrow_mut()
        //         .open_device(&device)
        //         .map_err(|e| format!("Error opening device: {e}"))?;
        //
        //     // For polling for drm::control::Event::PageFlip we need a blocking FD. Would be better to do this non-blocking
        //     let flags = nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFL)
        //         .map_err(|e| format!("Error getting file descriptor flags: {e}"))?;
        //     // Safetly: We only remove a bit, don't care about the others
        //     let mut flags = unsafe { nix::fcntl::OFlag::from_bits_unchecked(flags) };
        //     flags.remove(nix::fcntl::OFlag::O_NONBLOCK);
        //     nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_SETFL(flags))
        //         .map_err(|e| format!("Error making device fd non-blocking: {e}"))?;
        //
        //     // Safety: We take ownership of the now shared FD, ... although we should be using libseat's close_device....
        //     use std::os::fd::FromRawFd;
        //     Ok(Arc::new(unsafe { std::os::fd::OwnedFd::from_raw_fd(fd) }))
        // })?;
        // let renderer = (self.renderer_factory)(self.ohos_windows,self.width,self.height)?;

        let renderer=FemtoVGRendererAdapter::new(self.ohos_windows,self.width,self.height)?;

        let adapter = OhosWindowAdapter::new(renderer)?;
        // return Err(PlatformError::NoEventLoopProvider);

       *self.window.borrow_mut() = Some(adapter.clone());

        Ok(adapter)
    }

    fn run_event_loop(&self) -> Result<(), PlatformError> {
        // return Err(PlatformError::from("错误".to_string()));
        // let adapter = self.window.borrow().as_ref().unwrap().clone();
        let adapter = self.window.borrow().as_ref().ok_or_else(|| format!("Error windows adapter "))?.clone();

        let mut event_loop: EventLoop<LoopData> =
            EventLoop::try_new().map_err(|e| format!("Error creating event loop: {}", e))?;

        let loop_signal = event_loop.get_signal();

        *self.proxy.loop_signal.lock().unwrap() = Some(loop_signal.clone());
        let quit_loop = self.proxy.quit_loop.clone();

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
                format!("Re-entering the linuxkms event loop is currently not supported").into()
            );
        };

        event_loop
            .handle()
            .insert_source(user_event_receiver, |event, _, _| {
                let calloop::channel::Event::Msg(callback) = event else { return; };
                callback();
            })
            .map_err(
                |e: calloop::InsertError<calloop::channel::Channel<Box<dyn FnOnce() + Send>>>| {
                    format!("Error registering user event channel source: {e}")
                },
            )?;

        let mut loop_data = LoopData::default();

        quit_loop.store(false, std::sync::atomic::Ordering::Release);

        while !quit_loop.load(std::sync::atomic::Ordering::Acquire) {
            i_slint_core::platform::update_timers_and_animations();

            // adapter.render_if_needed(mouse_position_property.as_ref())?;
            adapter.render_if_needed()?;

            let next_timeout = if adapter.window().has_active_animations() {
                Some(std::time::Duration::from_millis(16))
            } else {
                i_slint_core::platform::duration_until_next_timer_update()
            };

            event_loop
                .dispatch(next_timeout, &mut loop_data)
                .map_err(|e| format!("Error dispatch events: {e}"))?;
        }

        Ok(())
    }

    fn new_event_loop_proxy(&self) -> Option<Box<dyn i_slint_core::platform::EventLoopProxy>> {
        Some(Box::new(self.proxy.clone()))
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
    fn clipboard_text(&self, clipboard: i_slint_core::platform::Clipboard) -> Option<String> {
        match clipboard {
            i_slint_core::platform::Clipboard::DefaultClipboard => self.clipboard.borrow().clone(),
            i_slint_core::platform::Clipboard::SelectionClipboard => {
                self.sel_clipboard.borrow().clone()
            }
            _ => None,
        }
    }
}

#[derive(Default)]
struct LoopData {}

struct Device {
    // in the future, use this from libseat: device_id: i32,
    fd: RawFd,
}

impl AsFd for Device {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        unsafe { BorrowedFd::borrow_raw(self.fd) }
    }
}
