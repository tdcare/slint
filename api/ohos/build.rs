
extern crate napi_build;
use std::env;
use std::fs::File;
use std::path::PathBuf;

use cfg_aliases::cfg_aliases;
use gl_generator::{Api, Fallbacks, Profile, Registry, StructGenerator};


fn main() {
  napi_build::setup();

  slint_build::compile_with_config(
    "booker.slint",
    slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".to_owned())
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
  )
      .unwrap();




  // XXX this is taken from glutin/build.rs.

  // Setup alias to reduce `cfg` boilerplate.
  cfg_aliases! {
        // Systems.
        // android_platform: { target_os = "android" },
        android_platform: { all(any(linux, unix)) },

        wasm_platform: { target_family = "wasm" },
        macos_platform: { target_os = "macos" },
        ios_platform: { target_os = "ios" },
        apple: { any(ios_platform, macos_platform) },
        free_unix: { all(unix, not(apple), not(android_platform)) },

        // Native displays.
        x11_platform: { all(feature = "x11", free_unix, not(wasm_platform)) },
        wayland_platform: { all(feature = "wayland", free_unix, not(wasm_platform)) },

        // Backends. feature = "egl",
        egl_backend: { all(any(windows, unix), not(apple), not(wasm_platform)) },
        glx_backend: { all(feature = "glx", x11_platform, not(wasm_platform)) },
        wgl_backend: { all(feature = "wgl", windows, not(wasm_platform)) },
        cgl_backend: { all(macos_platform, not(wasm_platform)) },
    }

  let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

  println!("cargo:rerun-if-changed=build.rs");

  let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
  Registry::new(Api::Gles2, (3, 0), Profile::Core, Fallbacks::All, [])
      .write_bindings(StructGenerator, &mut file)
      .unwrap();

}
