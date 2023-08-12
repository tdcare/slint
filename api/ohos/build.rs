
extern crate napi_build;

fn main() {
  napi_build::setup();

  slint_build::compile_with_config(
    "demo.slint",
    slint_build::CompilerConfiguration::new()
        .with_style("fluent-dark".to_owned())
        .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
  )
      .unwrap();
}
