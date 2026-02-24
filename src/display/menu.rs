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

pub fn main_menu(display:&mut OledDisplay, buttons: &Buttons) -> MenuChoice {
    display.clear();
    let heading_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();
    let options_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();
    
    let mut option:i32 = 0;
    
    let y_cod_of_first_option = 40;
    let y_cod_of_second_option = 55;

    // to track previous state so longpress != spamming
    //let mut prev_btn_state = crate::input::buttons::ButtonState::default(); 
    
    loop{
        display.clear(); // this first , hehehe
        //btns and logic here up in the loop
        // basically backend first
        let btn_state = buttons.read();

        // LOG EVERY FRAME so you can see if the loop is even running
        log::info!("Loop tick - option: {}", option);
        log::info!("Buttons - up:{} down:{} x:{} y:{}", 
            btn_state.up, btn_state.down, btn_state.x, btn_state.y);
        if btn_state.up {
            if option > 0 { option-=1; }
        }
        if btn_state.down {
            if option < 1 { option+=1; }            // also change here if max no. of options increase later for now there is only 2
        }

        if btn_state.x {
            //idea is to take value of option and match it
            return match option {
                0 => MenuChoice::Bluetooth,
                1 => MenuChoice::Games, 
                _ => MenuChoice::Bluetooth,
            }
        }

        //prev_btn_state = btn_state;


        // all the output/ drawing related stuff down here

        Text::new(
            "GAMING.....",
            Point::new(20,20),
            heading_style,
        ).draw(display).unwrap();

        Text::new( "bluetooth", 
            Point::new(40,y_cod_of_first_option),
            options_style
        ).draw(display).unwrap();
        
        Text::new( "games",
            Point::new(40,y_cod_of_second_option), 
            options_style
        ).draw(display).unwrap();

        if option == 0{
            Text::new(
                "==>",
                Point::new(10,y_cod_of_first_option),
                options_style
            ).draw(display).unwrap();
        
        }else if option == 1{
        
            Text::new(
                "==>",
                Point::new(10,y_cod_of_second_option),
                options_style
            ).draw(display).unwrap();
            
        }
        
        display.flush();
        
        FreeRtos::delay_ms(50);

    }
}