use slint::{platform::software_renderer, PlatformError, SharedString};


use core::{slice, time::Duration};
use std::rc::Rc;
use std::os::raw::c_void;
use std::sync::Mutex;


pub type TargetPixel = slint::platform::software_renderer::Rgb565Pixel;

// 用来向C++传数据的合局变量
pub static FRAME_BUFFER: once_cell::sync::OnceCell<Mutex<Vec<TargetPixel>>> = once_cell::sync::OnceCell::new();


pub struct Backend {
    ohos_windows: *mut c_void,
    width: u32,
    height: u32,
    window: Rc<software_renderer::MinimalSoftwareWindow>,
}

 impl  Backend {
    pub fn new(ohos_windows: *mut c_void, width: u32, height: u32, ) -> Result<Self, PlatformError> {
        Ok(Backend {
            ohos_windows,
            width,
            height,
            window: software_renderer::MinimalSoftwareWindow::new(
                software_renderer::RepaintBufferType::ReusedBuffer,
            ),
        })
    }
}



impl slint::platform::Platform for Backend {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }


    fn run_event_loop(&self) -> Result<(), slint::PlatformError> {
        // 用来接收显示图像的临时数组
        let mut fb = Vec::<TargetPixel>::with_capacity(self.width*self.height);
        FRAME_BUFFER.get_or_init(||{
            let buffer=Vec::<TargetPixel>::with_capacity(self.width*self.height);
            Mutex::new(buffer)
        });

        self.window.set_size(slint::PhysicalSize::new(
            self.width,
            self.height,
        ));

        loop {
            slint::platform::update_timers_and_animations();
            self.window.draw_if_needed(|renderer| {
                renderer.render(&mut fb, self.width.try_into().unwrap());

                // SAFETY: SlintBltPixel is a repr(transparent) BltPixel so it is safe to transform.
                // let blt_fb =
                //     unsafe { slice::from_raw_parts(fb.as_ptr() as *const BltPixel, fb.len()) };
                // 在这里想办法将fb 数组的内容传以ohos
                if let Some(ohos_buffer)= FRAME_BUFFER.get(){
                  if let Ok(mut data)= ohos_buffer.lock(){
                      data.clone_from(&fb);
                  }
                }

            });
            if !self.window.has_active_animations() {
               slint::platform::duration_until_next_timer_update();
            }
        }
    }
}
