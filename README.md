# OpenRTX Tool
OpenRTX swiss army knife

## Usage
* Install the rust toolchain using [rustup](https://rustup.rs/)
* This project required Cargo >= 1.59.0, check your version with `cargo --version`, if it is lower, install the nightly toolchain from rustup
* Clone this repository with the `--recursive` option and enter the `openrtxtool` folder
* Install build dependencies `sudo dnf install libusb-devel`
* Run `cargo run` to compile and run a debug build \
You can append rtxlink parameters to the `cargo run` command.
