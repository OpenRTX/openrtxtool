// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

use std::process;

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("rtxtool/radio_tool/include/radio_tool/radio/radio_factory.hpp");
        include!("rtxtool/include/radio_tool.h");
        fn flash_radio() -> Result<()>;
    }
}

fn main() {
    if let Err(err) = ffi::flash_radio() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
    println!("Firmware flashed, please reboot the radio");
}
