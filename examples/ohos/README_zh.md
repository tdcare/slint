# 提灯医疗操作系统V4.0(基于开源鸿蒙构建) 演示

演示Slint如何支持鸿蒙系统

![Screenshot](https://user-images.githubusercontent.com/1486/231705364-8c490e25-48cf-4626-a34b-2bf7239c1245.jpg)

## 技术路线
1. 编写一个鸿蒙系统后端，参考Slint Qt winint 以及 mcu-board 的实现。
2. 调用鸿蒙C++实现的 图像子系统 编入子系统 等子系统中的方法。
3. 用 libloading = { version = "0.8.0" } create 加载子系统中的方法。
4. 用 zigbuild 进行跨平台编译成能在鸿蒙下运行的运行程序。

```
rustup target install x86_64-unknown-uefi
```

To build, simply pass the `--package` and `--target` arguments to cargo:

```
cargo build --package uefi-demo --target x86_64-unknown-uefi
```

The produced UEFI binary can then either be tested on real hardware by booting
it like any other bootloader or directly with QEMU (the firmware location
varies by distro):

```
qemu-system-x86_64 -bios /usr/share/edk2-ovmf/x64/OVMF.fd -kernel target/x86_64-unknown-uefi/debug/uefi-demo.efi
```
