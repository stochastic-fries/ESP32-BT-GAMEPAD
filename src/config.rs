
#[derive(Debug, Clone, PartialEq)]
pub enum MenuChoice {
    Games,
    Bluetooth,
    Settings,
}
/*
pub enum Games{
    Snake,
    Debug,
}
    */
// pin numbers but are not being fully used in the 
// project 'cause the are input is req. as a IO pin not as an interger
pub const BTN_X_PIN: i32 = 34;
pub const BTN_Y_PIN: i32 = 35;
pub const BTN_A_PIN: i32 = 35;
pub const BTN_V_PIN: i32 = 35;

pub const BTN_UP_PIN: i32 = 32;
pub const BTN_DOWN_PIN: i32 = 33;
pub const BTN_LEFT_PIN: i32 = 25;
pub const BTN_RIGHT_PIN: i32 = 26;

pub const JOYSTICK_LEFT_X_PIN : i32 = 33;
pub const JOYSTICK_LEFT_Y_PIN : i32 = 33;
pub const JOYSTICK_RIGHT_X_PIN : i32 = 33;
pub const JOYSTICK_RIGHT_Y_PIN : i32 = 33;

pub const BTN_R1_PIN : i32 = 33;
pub const BTN_R2_PIN : i32 = 33;
pub const BTN_L1_PIN : i32 = 33;
pub const BTN_L2_PIN : i32 = 33;

//l3 and r3 don't have pin,   it works when l/r 1and2 are pressed together
// Display config
pub const DISPLAY_WIDTH: u32 = 128;
pub const DISPLAY_HEIGHT: u32 = 64;

