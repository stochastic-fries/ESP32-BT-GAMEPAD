use embedded_graphics::{
    mono_font::{ascii::FONT_9X18_BOLD, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use crate::display::OledDisplay;

pub fn show(display: &mut OledDisplay) {
    display.clear();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    Text::with_alignment(
        "Esp32 Gamepad!",
        Point::new(64, 32), // center of 128x64
        text_style,
        Alignment::Center,
    ).draw(display).unwrap();
    Text::with_alignment(
        "Loading....",
        Point::new(80,50),
        text_style,
        Alignment::Center,
    )
    .draw(display)
    .unwrap();

    display.flush().unwrap();

    std::thread::sleep(std::time::Duration::from_secs(2));
}