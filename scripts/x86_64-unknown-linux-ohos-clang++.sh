#!/bin/sh
exec /root/ohos-sdk/linux/native/llvm/bin/clang++ \
  -target x86_64-linux-ohos \
  --sysroot=/root/ohos-sdk/linux/native/sysroot \
  -D__MUSL__ \
  "$@"

