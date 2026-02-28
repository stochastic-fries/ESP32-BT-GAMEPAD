//this will help with debugging parts and to ensure all the inputs are working 
// this is not a game but a tool but may be required in future even after completing the project so 
// so it had to be somewhere permanent so i decided to put it in games folder

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
use crate::input::joysticks::Joysticks;
use esp_idf_svc::hal::delay::FreeRtos;

pub fn start(display: &mut OledDisplay, buttons: &Buttons, joysticks: &mut Joysticks){
    display.clear();
    let heading_style = MonoTextStyleBuilder::new()
    .font(&FONT_9X18_BOLD)
    .text_color(BinaryColor::On)
    .build();
    
    let options_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();
    let small_style = MonoTextStyleBuilder::new()
    .font(&FONT_4X6)
    .text_color(BinaryColor::On)
    .build();
    loop{
        let btn_states = buttons.read();
        let left_joystick = joysticks.read_left();
        let right_joystick = joysticks.read_right();
        
        display.clear();

        let debug_text = format!("x :{}, y  :{}, a  :{},\n b  :{} up:  {},\n down:  {},\n, left:{}, right:{} \n l1:{},l2:{}, r1:{}, r2:{}\n lx :{}, ly :{}\n rx :{}, ry :{}  ",
         btn_states.x, btn_states.y,btn_states.a, btn_states.b, btn_states.up,btn_states.down, btn_states.left,
          btn_states.right, btn_states.l1, btn_states.l2, btn_states.r1, btn_states.r2, left_joystick.x , left_joystick.y, 
          right_joystick.x, right_joystick.y);
        
        Text::new(
            &debug_text,
            Point::new(10,10),
            small_style,
        ).draw(display).unwrap();
        
        display.flush();
        
        FreeRtos::delay_ms(20);
        if btn_states.b{
            break
        }

    }
}