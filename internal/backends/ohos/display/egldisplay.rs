// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

use std::cell::Cell;
use std::os::raw::c_void;
// use std::os::fd::{AsFd, BorrowedFd};
use std::sync::Arc;
use std::cell::Cell;
use std::rc::{Rc, Weak};

// use crate::DeviceOpener;
// use drm::control::Device;
// use gbm::AsRaw;
use i_slint_core::api::PhysicalSize as PhysicalWindowSize;
use i_slint_core::platform::PlatformError;
use super::Presenter;


// Wrapped needed because gbm::Device<T> wants T to be sized.
// #[derive(Clone)]
// pub struct SharedFd(Arc<dyn AsFd>);
// impl AsFd for SharedFd {
//     fn as_fd(&self) -> BorrowedFd<'_> {
//         self.0.as_fd()
//     }
// }
//
// impl drm::Device for SharedFd {}
//
// impl drm::control::Device for SharedFd {}
//
// struct OwnedFramebufferHandle {
//     handle: drm::control::framebuffer::Handle,
//     device: SharedFd,
// }
//
// impl Drop for OwnedFramebufferHandle {
//     fn drop(&mut self) {
//         self.device.destroy_framebuffer(self.handle).ok();
//     }
// }

pub struct EglDisplay {
    pub oh_native_window:*mut std::os::raw::c_void,
    pub size: PhysicalWindowSize,
    pub presenter: Rc<dyn Presenter>,
}


impl raw_window_handle::HasWindowHandle for EglDisplay {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        let mut oh_surface_handle = raw_window_handle::OHOSWindowHandle::empty();
        // oh_surface_handle.a_native_window = self.oh_native_window.as_raw() as _;
        oh_surface_handle.a_native_window = self.oh_native_window;

        // Safety: This is safe because the handle remains valid; the next rwh release provides `new()` without unsafe.
        let active_handle = unsafe { raw_window_handle::ActiveHandle::new_unchecked() };

        Ok(unsafe {
            raw_window_handle::WindowHandle::borrow_raw(
                raw_window_handle::RawWindowHandle::OHOS(oh_surface_handle),
                active_handle,
            )
        })
    }
}

impl raw_window_handle::HasDisplayHandle for EglDisplay {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        let mut oh_display_handle = raw_window_handle::OHOSDisplayHandle::empty();
        // gbm_display_handle.gbm_device = self.display.as_raw() as _;

        Ok(unsafe {
            raw_window_handle::DisplayHandle::borrow_raw(raw_window_handle::RawDisplayHandle::OHOS(
                oh_display_handle,
            ))
        })
    }
}

pub fn create_egl_display(oh_native_window:*mut c_void,width:u32,height:u32) -> Result<EglDisplay, PlatformError> {
    let window_size = PhysicalWindowSize::new(width, height);
    Ok(EglDisplay {
        oh_native_window,
        size: window_size,
        presenter: TimerBasedAnimationDriver::new(),
    })
}



struct TimerBasedAnimationDriver {
    timer: i_slint_core::timers::Timer,
    next_animation_frame_callback: Cell<Option<Box<dyn FnOnce()>>>,
}

impl TimerBasedAnimationDriver {
    fn new() -> Rc<Self> {
        Rc::new_cyclic(|self_weak: &Weak<Self>| {
            let self_weak = self_weak.clone();
            let timer = i_slint_core::timers::Timer::default();
            timer.start(
                i_slint_core::timers::TimerMode::Repeated,
                std::time::Duration::from_millis(16),
                move || {
                    let Some(this) = self_weak.upgrade() else { return };
                    // Stop the timer and let the callback decide if we need to continue. It will set
                    // `needs_redraw` to true of animations should continue, render() will be called,
                    // present_with_next_frame_callback() will be called and then the timer restarted.
                    this.timer.stop();
                    if let Some(next_animation_frame_callback) =
                        this.next_animation_frame_callback.take()
                    {
                        next_animation_frame_callback();
                    }
                },
            );
            // Activate it only when we present a frame.
            timer.stop();

            Self { timer, next_animation_frame_callback: Default::default() }
        })
    }
}

impl Presenter for TimerBasedAnimationDriver {
    fn is_ready_to_present(&self) -> bool {
        true
    }

    fn register_page_flip_handler(
        &self,
        _event_loop_handle: crate::calloop_backend::EventLoopHandle,
    ) -> Result<(), PlatformError> {
        Ok(())
    }

    fn present_with_next_frame_callback(
        &self,
        ready_for_next_animation_frame: Box<dyn FnOnce()>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.next_animation_frame_callback.set(Some(ready_for_next_animation_frame));
        self.timer.restart();
        Ok(())
    }
}
