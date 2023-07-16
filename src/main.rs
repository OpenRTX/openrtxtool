// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

use adw::prelude::*;
use adw::{glib, Application, Leaflet, HeaderBar};
use gtk::{ApplicationWindow, Box, ListBox, Orientation, SelectionMode, ListBoxRow, Label};

const APP_ID: &str = "org.openrtx.openrtxtool";

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("openrtxtool/include/radio_tool.h");
        fn list_devices();
        fn flash_radio() -> Result<()>;
    }
}

fn install() {
    println!("Flashing OpenRTX firmware");
    if let Err(err) = ffi::flash_radio() {
        eprintln!("Error: {}", err);
        // process::exit(1);
    }
    println!("Firmware flash completed");
    println!("Please reboot the radio");
}


fn populate_device_list(list: &ListBox) -> &ListBox {
    //let devices = ffi::list_devices();
    let row = ListBoxRow::builder()
        .child(
            &Label::builder()
            .label("Test radio")
            .build(),
        )
        .build();
    list.append(&row);
    return list;
}

fn build_ui(app: &Application) {

    let leaflet = Leaflet::builder()
        .build();

    let device_list = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .build();

    populate_device_list(&device_list);
    leaflet.append(&device_list);

    let stack_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    leaflet.append(&stack_box);

    let window: ApplicationWindow = ApplicationWindow::builder()
        .application(app)
        .title("OpenRTX Tool")
        .default_height(600)
        .default_width(1000)
        .child(&leaflet)
        .build();

    window.show();
}

fn main() {
    // TODO: switch to inotify-rs to avoid polling on Linux

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}
