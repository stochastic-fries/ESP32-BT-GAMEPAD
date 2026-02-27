use embedded_graphics::{
    mono_font::{
        ascii::{
            FONT_9X18_BOLD,
            FONT_6X10,
            FONT_4X6,
        },
        MonoTextStyleBuilder} , 
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use crate::display::OledDisplay;
use crate::input::buttons::Buttons;
use crate::config::MenuChoice;
use esp_idf_svc::hal::delay::FreeRtos;


pub fn settings_menu(display:&mut OledDisplay, buttons: &Buttons){
    display.clear();

    let heading_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();
    
    let options_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    loop{
        let button_states = buttons.read();
        if button_states.b {break;}
        Text::with_alignment("SETTINGS",
            Point::new(64,10),
            options_style,
            Alignment::Center,
            ).draw(display).unwrap();
        
        display.flush();
        std::thread::sleep(std::time::Duration::from_millis(20));

    }
}