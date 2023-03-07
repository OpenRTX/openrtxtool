// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/radio_tool.cc")
        .file("radio_tool/src/ailunce_fw.cpp")
        .file("radio_tool/src/ailunce_radio.cpp")
        .file("radio_tool/src/dfu.cpp")
        .file("radio_tool/src/fymodem.c")
        .file("radio_tool/src/radio_factory.cpp")
        .file("radio_tool/src/tyt_device.cpp")
        .file("radio_tool/src/tyt_dfu.cpp")
        .file("radio_tool/src/tyt_fw.cpp")
        .file("radio_tool/src/tyt_radio.cpp")
        .file("radio_tool/src/usb_radio_factory.cpp")
        .file("radio_tool/src/serial_radio_factory.cpp")
        .file("radio_tool/src/ymodem_device.cpp")
        .file("radio_tool/src/yaesu_device.cpp")
        .file("radio_tool/src/yaesu_fw.cpp")
        .file("radio_tool/src/yaesu_radio.cpp")
        .file("radio_tool/src/h8sx.cpp")
        .include("include")
        .include("radio_tool/include")
        .flag_if_supported("-std=c++17")
        .compile("radio_factory");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/radio_tool.cc");
    println!("cargo:rerun-if-changed=include/radio_tool.h");
    println!("cargo:rustc-link-lib=usb-1.0");
}
