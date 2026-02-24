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

pub fn available_games(display:&mut OledDisplay, buttons: &Buttons)  {
    display.clear();
    let y_cod_of_first_option = 20;
    let menu_style = MonoTextStyleBuilder::new()
        .font(&FONT_4X6)
        .text_color(BinaryColor::On)
        .build();
    
    
    
    loop{
        display.clear()
        Text::new("debug", Point::new(20,y_cod_of_first_option),menu_style).draw(display).unwrap();
        button_states = buttons.read();
        
        
        






 
    
    
        display.flush();
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}