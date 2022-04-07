// src/main.rs
// Inspired by https://cxx.rs/tutorial.html

#[cxx::bridge(namespace = "radio_tool::radio")]
mod ffi {
    unsafe extern "C++" {
        include!("rtxtool/radio_tool/include/radio_tool/radio/radio_factory.hpp");
        include!("rtxtool/include/radio_factory.h");

        type RadioFactory;

        fn new_radiofactory() -> UniquePtr<RadioFactory>;
    }
}

fn main() {
    let client = ffi::new_radiofactory();
}
