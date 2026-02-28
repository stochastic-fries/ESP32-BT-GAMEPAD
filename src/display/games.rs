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
use esp_idf_svc::hal::delay::FreeRtos;
use crate::display::OledDisplay;
use crate::input::buttons::Buttons;
use crate::input::joysticks::Joysticks;
use crate::config::MenuChoice;
//use crate::config::Games;
use crate::games; // menu redirecing to choose games
pub fn available_games(display:&mut OledDisplay, buttons: &Buttons , joysticks: &mut Joysticks)  {
    display.clear();
    FreeRtos::delay_ms(200);

    let y_cod_of_first_option : i32 = 20;
    let y_cod_of_second_option: i32 = 40;
    let mut option = 0;
    let menu_style = MonoTextStyleBuilder::new()
        .font(&FONT_4X6)
        .text_color(BinaryColor::On)
        .build();                        
    
    
    let mut prev_btn_states = crate::input::buttons::ButtonState::default(); 
    let max_rows = 2;
    loop{
        display.clear();
        Text::new(
            "debug",
            Point::new(20,y_cod_of_first_option),
            menu_style
        ).draw(display).unwrap();
        Text::new("snak",
            Point::new(20,y_cod_of_second_option),
            menu_style
        ).draw(display).unwrap();
        
        
        let button_states = buttons.read();
        
        if button_states.up != prev_btn_states.up{
            if option > 0{
                option -=1;
            }
        }
        if button_states.down != prev_btn_states.down {
            if option< max_rows-1{      
                option+=1;
            }
        }

        if option == 0 {
            Text::new(
                "<==", 
                Point::new(80,y_cod_of_first_option),
                menu_style
            ).draw(display).unwrap();
        }
        if option == 1 {
            Text::new(
                "<==",
                Point::new(80,y_cod_of_second_option),
                menu_style
            ).draw(display).unwrap();
        }

        if button_states.x{
            match option {
                0 => {games::debug::start(display, &buttons, joysticks); },
                1 => games::snake::start(display, &buttons),
                _ => (),
            }
        }
        if button_states.b {
            break // to go back to main menu
        }
        display.flush();
        prev_btn_states = button_states;
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}