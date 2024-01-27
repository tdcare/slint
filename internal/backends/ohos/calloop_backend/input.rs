// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-Royalty-free-1.1 OR LicenseRef-Slint-commercial

//! This module contains the code to receive input events from libinput

use std::cell::{RefCell};
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, RecvError, SendError};
use calloop::channel::{Channel, Event, Sender};

use libc::*;
use i_slint_core::api::{EventLoopError, LogicalPosition};
use i_slint_core::platform::{PlatformError, PointerEventButton, WindowEvent};
use i_slint_core::{Property, SharedString};
use crate::calloop_backend::ohos::{OH_NativeXComponent_TouchEventType, OHOS_Input_Event};


pub enum  OHOSEventQueue {
    Queue(Vec<OHOS_Input_Event>),
}

impl OHOSEventQueue {
    pub fn send_event(&mut self, event: OHOS_Input_Event) -> Result<(), EventLoopError> {
        match self {
            OHOSEventQueue::Queue(queue) => {
                queue.push(event);
                Ok(())
            }
        }
    }

}

impl Default for OHOSEventQueue {
    fn default() -> Self {
        Self::Queue(Vec::new())
    }
}

pub static GLOBAL_PROXY: once_cell::sync::OnceCell<Mutex<OHOSEventQueue>> = once_cell::sync::OnceCell::new();
pub static OHOS_EVENT_SENDER: once_cell::sync::OnceCell<Mutex<Sender<OHOS_Input_Event>>> =  once_cell::sync::OnceCell::new();

pub struct OHOSInputHandler<'a> {
    token: Option<calloop::Token>,
    window: &'a i_slint_core::api::Window,
    // keystate: xkb::State,
}

impl<'a> OHOSInputHandler<'a> {
    pub fn init<T>(
        window: &'a i_slint_core::api::Window,
        event_loop_handle: &calloop::LoopHandle<'a, T>,
        ohos_event_receiver:Channel<OHOS_Input_Event>,
    ) -> Result<Pin<Rc<Property<Option<LogicalPosition>>>>, PlatformError> {


        let mouse_pos_property = Rc::pin(Property::new(None));

        let handler = Self {
            token: Default::default(),
            window,
        };

        event_loop_handle
            .insert_source(ohos_event_receiver, move |calloop_event, _, _| {
                match calloop_event {
                    Event::Msg(ohos_event) => {
                        let screen_size = window.size();
                        if  let Some(event)=match ohos_event {
                            OHOS_Input_Event::MouseEvent(_) => {
                             None
                            }
                            OHOS_Input_Event::TouchEvent(touch_event) => {
                               match touch_event.r#type{
                                   OH_NativeXComponent_TouchEventType::OH_NATIVEXCOMPONENT_DOWN => {
                                       let last_touch_pos = LogicalPosition::new(
                                           touch_event.x,
                                           touch_event.y,
                                       );
                                       Some(WindowEvent::PointerPressed {
                                           position: last_touch_pos,
                                           button: PointerEventButton::Left,
                                       })
                                   }
                                   OH_NativeXComponent_TouchEventType::OH_NATIVEXCOMPONENT_UP => {None}
                                   OH_NativeXComponent_TouchEventType::OH_NATIVEXCOMPONENT_MOVE => {None}
                                   OH_NativeXComponent_TouchEventType::OH_NATIVEXCOMPONENT_CANCEL => {None}
                                   OH_NativeXComponent_TouchEventType::OH_NATIVEXCOMPONENT_UNKNOWN => {None}
                               }


                            }
                            OHOS_Input_Event::NoEvent => {
                                None
                           }
                        } {
                            window.dispatch_event(event);
                        }
                    }
                    Event::Closed => {}
                }


            })
            .map_err(|e| format!("Error registering libinput event source: {e}"))?;
        Ok(mouse_pos_property)
    }
}


// fn map_key_sym(sym: u32) -> Option<SharedString> {
//     macro_rules! keysym_to_string {
//         ($($char:literal # $name:ident # $($_qt:ident)|* # $($_winit:ident)|* # $($xkb:ident)|*;)*) => {
//             match(sym) {
//                 $($(xkb::$xkb => $char,)*)*
//                 // _ => std::char::from_u32(xkbcommon::xkb::keysym_to_utf32(sym))?,
//                 _ => std::char::from_u32(sym)?,
//
//             }
//         };
//     }
//     let char = i_slint_common::for_each_special_keys!(keysym_to_string);
//     Some(char.into())
// }
