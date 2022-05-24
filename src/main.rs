// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

use std::process;
use std::time::Duration;
use std::thread;

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("rtxtool/radio_tool/include/radio_tool/radio/radio_factory.hpp");
        include!("rtxtool/include/radio_tool.h");
        fn flash_radio() -> Result<()>;
        fn reboot_radio() -> Result<()>;
    }
}

fn main() {
    if let Err(err) = ffi::flash_radio() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
    println!("Firmware flash completed");
    thread::sleep(Duration::from_millis(2000));
    println!("Rebooting radio");
    ffi::reboot_radio();
    // println!("Firmware flashed, please reboot the radio");
}
