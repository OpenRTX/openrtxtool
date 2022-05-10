// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("rtxtool/radio_tool/include/radio_tool/radio/radio_factory.hpp");
        include!("rtxtool/include/radio_tool.h");

        fn radio_tool_flash();
    }
}

fn main() {
    ffi::radio_tool_flash();
}
