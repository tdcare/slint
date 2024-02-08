// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT
use napi_build_ohos;
use napi_build;

fn main() {
    napi_build::setup();
    // napi_build_ohos::setup();

    // slint_build::compile_with_config(
    //     "demo.slint",
    //     slint_build::CompilerConfiguration::new()
    //         .with_style("fluent-dark".to_owned())
    //         .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    // )
    // .unwrap();
}
