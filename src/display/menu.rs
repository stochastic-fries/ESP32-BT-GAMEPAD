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
    // font of the selection arrow is changed only in the function which creates it

    let mut option:i32 = 0;
    
    let y_cod_of_first_option = 30;
    let y_cod_of_second_option = 45;
    let y_cod_of_third_option = 55;

    let max_rows = 3;
    
    // so longpress != spamming
    let mut prev_btn_state = crate::input::buttons::ButtonState::default(); 

    fn draw_selection_arrow(display: &mut OledDisplay ,y_cod_of_the_option:i32){
        let options_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .build();

        Text::new(
                "==>",
                Point::new(10,y_cod_of_the_option),
                options_style
            ).draw(display).unwrap();
            
    }
    
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
            if option < max_rows - 1 { option+=1; }           
        }


        if btn_state.x != prev_btn_state.x{
            //idea is to take value of option and match it
            return match option {
                0 => MenuChoice::Bluetooth,
                1 => MenuChoice::Games, 
                2 => MenuChoice::Settings,
                _ => MenuChoice::Bluetooth,
            }
        }

        prev_btn_state = btn_state;


        // all the output/ drawing related stuff down here

        Text::new(
            "GAMING.....",
            Point::new(20,10),
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

        Text::new( "settings",
            Point::new(40,y_cod_of_third_option),
            options_style
        ).draw(display).unwrap();
        
        match option{
            0 => draw_selection_arrow(display, y_cod_of_first_option),
            1 => draw_selection_arrow(display, y_cod_of_second_option),
            2 => draw_selection_arrow(display, y_cod_of_third_option),

            _ => draw_selection_arrow(display, y_cod_of_first_option),
        }
        
        display.flush();
        
        FreeRtos::delay_ms(100);

    }
}