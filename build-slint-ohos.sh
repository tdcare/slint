docker run --rm -it -v $(pwd):/io -w /io/api/ohos swr.cn-south-1.myhuaweicloud.com/tdcare/zig-build-slint:v1.70.0-libudev \
   cargo zigbuild --release --target aarch64-unknown-linux-musl