// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

use std::env;
use std::process;
use std::time::Duration;
use std::thread;

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("rtxtool/radio_tool/include/radio_tool/radio/radio_factory.hpp");
        include!("rtxtool/include/radio_tool.h");
        fn list_devices();
        fn flash_radio() -> Result<()>;
        fn reboot_radio() -> Result<()>;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Print usage information
    if args.len() < 2 {
        println!("rtxtool: OpenRTX Installation Software");
        println!("usage: {} COMMAND", args[0]);
        println!("commands:");
        println!(" list                       List the supported connected radios");
        println!(" install                    Install OpenRTX");
        println!(" restore                    Restore original firmware");
        process::exit(0);
    }
    let command = args[1].clone();
    if command == "list" { list(); }
    else if command == "install" { install(); }
}

fn list() {
    ffi::list_devices();
}

fn install() {
    println!("Flashing OpenRTX firmware");
    if let Err(err) = ffi::flash_radio() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
    println!("Firmware flash completed");
//    println!("Rebooting radio");
//    ffi::reboot_radio();
    println!("Please reboot the radio");
}
