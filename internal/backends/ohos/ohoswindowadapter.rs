// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

//! This module contains the window adapter implementation to communicate between Slint and Vulkan + libinput

use std::cell::Cell;
use std::pin::Pin;
use std::rc::Rc;
// use hilog_binding::hilog_debug;

use i_slint_core::api::{LogicalPosition, PhysicalSize as PhysicalWindowSize};
use i_slint_core::graphics::euclid;
use i_slint_core::graphics::Image;
use i_slint_core::item_rendering::ItemRenderer;
use i_slint_core::platform::WindowEvent;
use i_slint_core::slice::Slice;
use i_slint_core::Property;
use i_slint_core::{platform::PlatformError, window::WindowAdapter};
use crate::display::RenderingRotation;

pub trait OhosRenderer {
    // fn as_core_renderer(&self) -> &dyn i_slint_core::renderer::Renderer;
    // fn render_and_present(
    //     &self,
    //     draw_mouse_cursor_callback: &dyn Fn(&mut dyn ItemRenderer),
    // ) -> Result<(), PlatformError>;
    // fn size(&self) -> PhysicalWindowSize;

    fn as_core_renderer(&self) -> &dyn i_slint_core::renderer::Renderer;
    fn is_ready_to_present(&self) -> bool;
    fn render_and_present(
        &self,
        rotation: RenderingRotation,
        draw_mouse_cursor_callback: &dyn Fn(&mut dyn ItemRenderer),
        ready_for_next_animation_frame: Box<dyn FnOnce()>,
    ) -> Result<(), PlatformError>;
    fn size(&self) -> PhysicalWindowSize;
    fn register_page_flip_handler(
        &self,
        event_loop_handle: crate::calloop_backend::EventLoopHandle,
    ) -> Result<(), PlatformError>;
}

pub struct OhosWindowAdapter {
    window: i_slint_core::api::Window,
    renderer: Box<dyn OhosRenderer>,
    // needs_redraw: Cell<bool>,
    redraw_requested: Cell<bool>,
    needs_redraw_after_present: Cell<bool>,
    rotation: RenderingRotation,
    dark_color_scheme: Pin<Box<Property<bool>>>,
}

impl WindowAdapter for OhosWindowAdapter {
    fn window(&self) -> &i_slint_core::api::Window {
        &self.window
    }
    fn size(&self) -> i_slint_core::api::PhysicalSize {
        self.rotation.screen_size_to_rotated_window_size(self.renderer.size())
    }
    fn renderer(&self) -> &dyn i_slint_core::renderer::Renderer {
        self.renderer.as_core_renderer()
    }
    fn request_redraw(&self) {
        self.redraw_requested.set(true)
    }

    fn set_visible(&self, visible: bool) -> Result<(), PlatformError> {
        if visible {
            if let Some(scale_factor) =
                std::env::var("SLINT_SCALE_FACTOR").ok().and_then(|sf| sf.parse().ok())
            {
                self.window.dispatch_event(WindowEvent::ScaleFactorChanged { scale_factor });
            }
        }
        Ok(())
    }


    fn internal(
        &self,
        _: i_slint_core::InternalToken,
    ) -> Option<&dyn i_slint_core::window::WindowAdapterInternal> {
        Some(self)
    }
}
impl i_slint_core::window::WindowAdapterInternal for OhosWindowAdapter {

    fn input_method_request(&self, request: i_slint_core::window::InputMethodRequest) {

    }

    fn dark_color_scheme(&self) -> bool {
        self.dark_color_scheme.as_ref().get()
    }
}
impl OhosWindowAdapter {
    pub fn new(renderer: Box<dyn OhosRenderer>,
               rotation: RenderingRotation,
    ) -> Result<Rc<Self>, PlatformError> {
        let size = renderer.size();
        let rotation_degrees = rotation.degrees();
        eprintln!(
            "Rendering at {}x{}{}",
            size.width,
            size.height,
            if rotation_degrees != 0. {
                format!(" with {} rotation_degrees rotation", rotation_degrees)
            } else {
                String::new()
            }
        );


        Ok(Rc::<OhosWindowAdapter>::new_cyclic(|self_weak| OhosWindowAdapter {
            window: i_slint_core::api::Window::new(self_weak.clone()),
            renderer,
            redraw_requested: Cell::new(true),
            needs_redraw_after_present: Cell::new(false),
            rotation,
            dark_color_scheme:Box::pin(Property::new(true)),
        }))
    }

    pub fn render_if_needed(
        self: Rc<Self>,
        mouse_position: Pin<&Property<Option<LogicalPosition>>>,
    ) -> Result<(), PlatformError> {
        if !self.renderer.is_ready_to_present() {
            return Ok(());
        }
        if self.redraw_requested.replace(false) {
            self.renderer.render_and_present(
                self.rotation,
                &|item_renderer| {
                    // if let Some(mouse_position) = mouse_position.get() {
                    //     item_renderer.save_state();
                    //     item_renderer.translate(
                    //         i_slint_core::lengths::logical_point_from_api(mouse_position)
                    //             .to_vector(),
                    //     );
                    //     item_renderer.draw_image_direct(mouse_cursor_image());
                    //     item_renderer.restore_state();
                    // }
                },
                Box::new({
                   || return ;

                //     let self_weak = Rc::downgrade(&self);
                //     move || {
                //         let Some(this) = self_weak.upgrade() else {
                //             return;
                //         };
                //         if this.needs_redraw_after_present.replace(false) {
                //             this.request_redraw();
                //         }
                //     }
                //
                }
                ),
            )?;
            // Check once after rendering if we have running animations and
            // remember that to trigger a redraw after the frame is on the screen.
            // Timers might have been updated if the event loop is woken up
            // due to other reasons, which would also reset has_active_animations.
            self.needs_redraw_after_present.set(self.window.has_active_animations());
        }
        Ok(())
    }

    pub fn register_event_loop(
        &self,
        event_loop_handle: crate::calloop_backend::EventLoopHandle,
    ) -> Result<(), PlatformError> {
        self.renderer.register_page_flip_handler(event_loop_handle)
    }

}

fn mouse_cursor_image() -> Image {
    let mouse_pointer_svg = i_slint_core::graphics::load_image_from_embedded_data(
        Slice::from_slice(include_bytes!("mouse-pointer.svg")),
        Slice::from_slice(b"svg"),
    );
    let mouse_pointer_inner: &i_slint_core::graphics::ImageInner = (&mouse_pointer_svg).into();
    match mouse_pointer_inner {
        i_slint_core::ImageInner::Svg(svg) => {
            let pixels = svg.render(None).unwrap();
            let cache_key = svg.cache_key();
            let mouse_pointer_pixel_image = i_slint_core::graphics::ImageInner::EmbeddedImage {
                cache_key: cache_key.clone(),
                buffer: pixels,
            };
            i_slint_core::graphics::cache::replace_cached_image(
                cache_key,
                mouse_pointer_pixel_image.clone(),
            );

            mouse_pointer_pixel_image.into()
        }
        cached_image @ _ => cached_image.clone().into(),
    }
}
