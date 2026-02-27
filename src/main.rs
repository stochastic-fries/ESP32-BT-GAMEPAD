use esp_idf_svc::hal::{
    prelude::*,
    peripherals::Peripherals,
    gpio::IOPin,
    i2c::*,
    units::KiloHertz,

};
use sh1106::{Builder, mode::GraphicsMode};  // <-- sh1106 instead of ssd1306

mod config;
mod display;
mod input;
mod bluetooth;
mod games;

use input::buttons::Buttons;
use crate::config::MenuChoice;


fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    //display 
    // --- Set up I2C ---
    let sda = peripherals.pins.gpio21; // default I2C SDA on ESP32
    let scl = peripherals.pins.gpio22; // default I2C SCL on ESP32
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &I2cConfig::new().baudrate(KiloHertz(400).into()),
    ).unwrap();

    // --- Set up display ---
    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    display.init().unwrap();
    display.flush().unwrap();


    // Set up buttons using the actual pin objects
    // We use downgrade_input() to convert specific pin types into AnyInputPin
    let mut buttons = Buttons::new(
        peripherals.pins.gpio12.downgrade(), // x
        peripherals.pins.gpio14.downgrade(), // y
        peripherals.pins.gpio27.downgrade(), // a
        peripherals.pins.gpio13.downgrade(), // b

        peripherals.pins.gpio17.downgrade(), // up 
        peripherals.pins.gpio5.downgrade(), // down
        peripherals.pins.gpio18.downgrade(), // left
        peripherals.pins.gpio19.downgrade(), // right

        peripherals.pins.gpio23.downgrade(), // l1
        peripherals.pins.gpio25.downgrade(), // l2
        peripherals.pins.gpio0.downgrade(), // r1
        peripherals.pins.gpio2.downgrade(), // r2

        peripherals.pins.gpio16.downgrade(), // start 
        peripherals.pins.gpio4.downgrade(), // select
        peripherals.pins.gpio26.downgrade(), // back , temporary just to avoid un-necessary error

    );

    display::welcome::show(&mut display);
    loop{
        let choice = display::menu::main_menu(&mut display, &buttons);
        
        match choice {
            //MenuChoice::Bluetooth =>
            MenuChoice::Games =>display::games::available_games(&mut display, &buttons),
            MenuChoice::Settings =>display::settings::settings_menu(&mut display, &buttons),
            _ => display::games::available_games(&mut display, &buttons),
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
        
    }
}
