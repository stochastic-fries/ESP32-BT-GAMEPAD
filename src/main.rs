use esp_idf_svc::hal::{
    prelude::*,
    peripherals::Peripherals,
    gpio::IOPin,
    i2c::*,
    units::KiloHertz,

};
use sh1106::{Builder, mode::GraphicsMode};  

mod config;
mod display;
mod input;
mod bluetooth;
mod games;
use esp_idf_svc::hal::delay::FreeRtos;


use input::joysticks::Joysticks;
use input::buttons::Buttons;
use crate::config::MenuChoice;


fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::nvs::EspDefaultNvsPartition::take().unwrap();

    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();

    //display 
    // Set up I2C 
    let sda = peripherals.pins.gpio21; 
    let scl = peripherals.pins.gpio22; 
    let i2c = I2cDriver::new(
        peripherals.i2c0,
        sda,
        scl,
        &I2cConfig::new().baudrate(KiloHertz(400).into()),
    ).unwrap();

    // Set up display 
    let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    display.init().unwrap();
    display.flush().unwrap();


    // back buttons doesn't work with HID or bluetooth devices it is just to nevigate b/w menus
      let mut buttons = Buttons::new(
       
       
       peripherals.pins.gpio27.into(),  // col0
       peripherals.pins.gpio14.into(),  // col1
       peripherals.pins.gpio12.into(),  // col2
       peripherals.pins.gpio13.into(),  // col3
       
        peripherals.pins.gpio19.into(),  // row0
       peripherals.pins.gpio18.into(),  // row1
       peripherals.pins.gpio5.into(),  // row2
       peripherals.pins.gpio17.into(),  // row3
       
       
       peripherals.pins.gpio23.into(),  // back z
   );

    let mut joysticks = Joysticks::new(
    peripherals.adc1,
    peripherals.pins.gpio34, // left X
    peripherals.pins.gpio35, // left Y
    peripherals.pins.gpio32, // right X
    peripherals.pins.gpio33, // right Y
    );

    display::welcome::show(&mut display);
    
    buttons.debug_scan(); 
    loop {
       let choice = display::menu::main_menu(&mut display, &mut buttons);
       match choice {
           MenuChoice::Bluetooth => bluetooth::gamepad::start(&mut display, &mut buttons, &mut joysticks),
           MenuChoice::Games => display::games::available_games(&mut display, &mut buttons, &mut joysticks),
           MenuChoice::Settings => display::settings::settings_menu(&mut display, &mut buttons),
           _ => display::games::available_games(&mut display, &mut buttons, &mut joysticks),
       }
       FreeRtos::delay_ms(20);
}
}
