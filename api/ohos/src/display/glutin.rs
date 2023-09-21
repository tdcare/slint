use std::error::Error;
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::Deref;
use std::os::raw::c_void;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
    pub use Gles2 as Gl;
}

use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextApi, ContextAttributesBuilder, Version};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use raw_window_handle::{HasDisplayHandle, HasRawDisplayHandle, HasRawWindowHandle, HasWindowHandle};
use i_slint_core::platform::PlatformError;

pub fn glution_egl(ohos_widows: *mut c_void,w:u32,h:u32,)->Result<i32, PlatformError>{
    let egl_display = crate::display::egldisplay::create_egl_display(ohos_widows, w, h)?;
    let width: std::num::NonZeroU32 = egl_display.size.width.try_into().map_err(|_| {
        format!(
            "Attempting to create window surface with an invalid width: {}",
            egl_display.size.width
        )
    })?;
    let height: std::num::NonZeroU32 = egl_display.size.height.try_into().map_err(|_| {
        format!(
            "Attempting to create window surface with an invalid height: {}",
            egl_display.size.height
        )
    })?;

    let display_handle = egl_display.display_handle()
        .map_err(|e| format!("display_handle: {e}"))?;

    let window_handle = egl_display.window_handle()
        .map_err(|e| format!("window_handle: {e}"))?;


    let gl_display = unsafe {
        glutin::display::Display::new(
            display_handle.raw_display_handle(),
            glutin::display::DisplayApiPreference::Egl,
        )
            .map_err(|e| format!("Error creating EGL display: {e}"))?
    };

    let config_template = glutin::config::ConfigTemplateBuilder::new().build();

    let config = unsafe {
        gl_display
            .find_configs(config_template)
            .map_err(|e| format!("Error locating EGL configs: {e}"))?
            .reduce(|accum, config| {
                let transparency_check = config.supports_transparency().unwrap_or(false)
                    & !accum.supports_transparency().unwrap_or(false);

                if transparency_check || config.num_samples() < accum.num_samples() {
                    config
                } else {
                    accum
                }
            })
            .ok_or("Unable to find suitable GL config")?
    };

    let gles_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(Some(glutin::context::Version {
            major: 2,
            minor: 0,
        })))
        .build(Some(window_handle.raw_window_handle()));

    let fallback_context_attributes =
        ContextAttributesBuilder::new().build(Some(window_handle.raw_window_handle()));

    let not_current_gl_context = unsafe {
        gl_display
            .create_context(&config, &gles_context_attributes)
            .or_else(|_| gl_display.create_context(&config, &fallback_context_attributes))
            .map_err(|e| format!("Error creating EGL context: {e}"))?
    };

    let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
        window_handle.raw_window_handle(),
        width,
        height,
    );

    let surface = unsafe {
        config
            .display()
            .create_window_surface(&config, &attrs)
            .map_err(|e| format!("Error creating EGL window surface: {e}"))?
    };

    let context = not_current_gl_context.make_current(&surface)
        .map_err(|glutin_error: glutin::error::Error| -> PlatformError {
            format!("FemtoVG Renderer: Failed to make newly created OpenGL context current: {glutin_error}")
                .into()
        })?;

    // drop(window_handle);
    // drop(display_handle);

    let renderer=Renderer::new(&gl_display);
    surface.resize(
        &context,
        NonZeroU32::new(w).unwrap(),
        NonZeroU32::new(h).unwrap(),
    );
    renderer.resize(w as i32,h as i32);
    renderer.draw();
    surface.swap_buffers(&context).unwrap();

    Ok(1)
}



pub struct Renderer {
    program: gl::types::GLuint,
    vao: gl::types::GLuint,
    vbo: gl::types::GLuint,
    gl: gl::Gl,
}

impl Renderer {
    pub fn new<D: GlDisplay>(gl_display: &D) -> Self {
        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            let vertex_shader = create_shader(&gl, gl::VERTEX_SHADER, VERTEX_SHADER_SOURCE);
            let fragment_shader = create_shader(&gl, gl::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE);

            let program = gl.CreateProgram();

            gl.AttachShader(program, vertex_shader);
            gl.AttachShader(program, fragment_shader);

            gl.LinkProgram(program);

            gl.UseProgram(program);

            gl.DeleteShader(vertex_shader);
            gl.DeleteShader(fragment_shader);

            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);

            let mut vbo = std::mem::zeroed();
            gl.GenBuffers(1, &mut vbo);
            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                VERTEX_DATA.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            let pos_attrib = gl.GetAttribLocation(program, b"position\0".as_ptr() as *const _);
            let color_attrib = gl.GetAttribLocation(program, b"color\0".as_ptr() as *const _);
            gl.VertexAttribPointer(
                pos_attrib as gl::types::GLuint,
                2,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                std::ptr::null(),
            );
            gl.VertexAttribPointer(
                color_attrib as gl::types::GLuint,
                3,
                gl::FLOAT,
                0,
                5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
                (2 * std::mem::size_of::<f32>()) as *const () as *const _,
            );
            gl.EnableVertexAttribArray(pos_attrib as gl::types::GLuint);
            gl.EnableVertexAttribArray(color_attrib as gl::types::GLuint);

            Self { program, vao, vbo, gl }
        }
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.UseProgram(self.program);

            self.gl.BindVertexArray(self.vao);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.ClearColor(0.1, 0.1, 0.1, 0.9);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.program);
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

unsafe fn create_shader(
    gl: &gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(shader, 1, [source.as_ptr().cast()].as_ptr(), std::ptr::null());
    gl.CompileShader(shader);
    shader
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
    0.0,  0.5,  0.0,  1.0,  0.0,
    0.5, -0.5,  0.0,  0.0,  1.0,
];

const VERTEX_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

attribute vec2 position;
attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
\0";

const FRAGMENT_SHADER_SOURCE: &[u8] = b"
#version 100
precision mediump float;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";