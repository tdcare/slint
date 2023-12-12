#!/bin/sh
exec /root/ohos-sdk/linux/native/llvm/bin/clang \
  -target arm-linux-ohos \
  --sysroot=/root/ohos-sdk/linux/native/sysroot \
  -D__MUSL__ \
  -march=armv7-a \
  -mfloat-abi=softfp \
  -mtune=generic-armv7-a \
  -mthumb \
  "$@"
