// build.rs

fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/radio_tool.cc")
        .file("radio_tool/src/radio_factory.cpp")
        .file("radio_tool/src/usb_radio_factory.cpp")
        .file("radio_tool/src/serial_radio_factory.cpp")
        .include("include")
        .include("radio_tool/include")
        .flag_if_supported("-std=c++17")
        .flag_if_supported("-lusb-1.0")
        .compile("radio_factory");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=radio_tool/src/radio_factory.cpp");
    println!("cargo:rerun-if-changed=radio_tool/src/usb_radio_factory.cpp");
    println!("cargo:rerun-if-changed=radio_tool/src/serial_radio_factory.cpp");
    println!("cargo:rerun-if-changed=radio_tool/include/radio_tool/radio/radio_factory.hpp");
    println!("cargo:rerun-if-changed=radio_tool/include/radio_tool/radio/usb_radio_factory.hpp");
    println!("cargo:rerun-if-changed=radio_tool/include/radio_tool/radio/serial_radio_factory.hpp");
}
