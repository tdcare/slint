# 提灯医疗操作系统V4.0(基于开源鸿蒙构建) 演示

演示Slint如何支持鸿蒙系统

![Screenshot](https://user-images.githubusercontent.com/1486/231705364-8c490e25-48cf-4626-a34b-2bf7239c1245.jpg)

## 技术路线
1. 编写一个鸿蒙系统后端，参考Slint Qt winint 以及 mcu-board 的实现。
2. 调用鸿蒙C++实现的 图像子系统 编入子系统 等子系统中的方法。
~~3. 用 libloading = { version = "0.8.0" } create 加载子系统中的方法。(这个办法行不通)~~
3. 结合Slint-winit与鸿蒙Native C++ 示例中关于事件传递方法(同时还可以参考Qt for OpenHarmony 项目)，有以下关键点: 
    * 参考 Slint-winint 中的 Eventloop 实现方法：用一个OneCell保存一个全局事件队列，event_loop run 方法中，从队列中取到事件，发送到Slint中。
    * 用Rust实现一个供C++调用的函数，在鸿蒙的Native C++代码中，注册为事件回调函数，用于接收鸿蒙的事件，接收到事件后，发给全局事件队列。
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
