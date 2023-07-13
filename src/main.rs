// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

use std::env;
use std::process;
use std::time::Duration;
use std::thread;
use adw::prelude::*;
use adw::{Application, ApplicationWindow, HeaderBar};
use gtk::{Box, Orientation, SelectionMode, Label, Button, Picture};

const APP_ID: &str = "org.openrtx.openrtxtool";

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("openrtxtool/include/radio_tool.h");
        fn list_devices();
        fn flash_radio() -> Result<()>;
    }
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
    println!("Please reboot the radio");
}

fn build_ui(app: &Application) {
    let logo = Picture::for_filename("./img/openrtx_logo.svg");
    logo.set_alternative_text(Some("OpenRTX"));
    logo.set_margin_top(100);

    let label = Label::builder()
        .margin_top(230)
        .margin_end(32)
        .margin_bottom(230)
        .margin_start(32)
        .label("Connect your radio")
        .build();

    //let button = Button::builder()
    //    .label("Flash")
    //    .margin_top(12)
    //    .margin_bottom(12)
    //    .margin_start(50)
    //    .margin_end(50)
    //    .build();

    // Combine the content in a box
    let content = Box::new(Orientation::Vertical, 0);
    // Adwaitas' ApplicationWindow does not include a HeaderBar
    content.append(&HeaderBar::new());
    content.append(&logo);
    content.append(&label);
    //content.append(&button);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("openrtxtool")
        .default_height(600)
        .default_width(1000)
        .content(&content)
        .build();

    window.show();
}

fn main() {
    // Spawn a new thread to monitor for new radio connections
    // TODO: switch to inotify-rs to avoid polling on Linux
    thread::spawn(|| {
        list();
    });

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
    //let args: Vec<String> = env::args().collect();

    //// Print usage information
    //if args.len() < 2 {
    //    println!("openrtxtool: OpenRTX Installation Software");
    //    println!("usage: {} COMMAND", args[0]);
    //    println!("commands:");
    //    println!(" list                       List the supported connected radios");
    //    println!(" install                    Install OpenRTX");
    //    println!(" restore                    Restore original firmware");
    //    process::exit(0);
    //}
    list();

    //match args[1]
    //    .clone()
    //    .as_str() {
    //    "list" => list(),
    //    "install" => install(),
    //    "restore" => todo!(),
    //    _ => todo!(),
    //}
}
