// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
// 
// the keyboard HID code is taken from a github page 
// https://github.com/taks/esp32-nimble/blob/main/examples/ble_keyboard.rs , 
// and claude re wrote the HID profile for gamepad and then i editted it , to make it work

#![allow(dead_code)]

use esp32_nimble::{
  BLEAdvertisementData, BLECharacteristic, BLEDevice, BLEHIDDevice, BLEServer, enums::*, hid::*,
  utilities::mutex::Mutex,
};
use std::sync::Arc;
use zerocopy::IntoBytes;
use zerocopy_derive::{Immutable, IntoBytes};
use esp_idf_svc::hal::delay::FreeRtos;


use crate::display::OledDisplay;
use crate::input::buttons::Buttons;
use crate::input::joysticks::Joysticks;



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


const GAMEPAD_ID: u8 = 0x01;
const HID_REPORT_DESCRIPTOR: &[u8] = hid!(
  (USAGE_PAGE, 0x01),
  (USAGE, 0x05),           // Gamepad
  (COLLECTION, 0x01),
  (REPORT_ID, GAMEPAD_ID),
  // 16 buttons, fits u16 perfectly, no padding needed
  (USAGE_PAGE, 0x09),
  (USAGE_MINIMUM, 0x01),
  (USAGE_MAXIMUM, 0x10),   // 16 buttons
  (LOGICAL_MINIMUM, 0x00),
  (LOGICAL_MAXIMUM, 0x01),
  (REPORT_SIZE, 0x01),
  (REPORT_COUNT, 0x10),    // 16 bits
  (HIDINPUT, 0x02),
  // 4 axes (LX, LY, RX, RY)
  (USAGE_PAGE, 0x01),
  (USAGE, 0x30),           // X  (left stick horizontal)
  (USAGE, 0x31),           // Y  (left stick vertical)
  (USAGE, 0x32),           // Z  (right stick horizontal)
  (USAGE, 0x35),           // Rz (right stick vertical)
  (LOGICAL_MINIMUM, 0x81), // -127
  (LOGICAL_MAXIMUM, 0x7F), // 127
  (REPORT_SIZE, 0x08),
  (REPORT_COUNT, 0x04),
  (HIDINPUT, 0x02),
  (END_COLLECTION),
);
// Gamepad buttons bitmask constants
pub struct Button;
impl Button {
    // Action
    pub const A: u16 = 1 << 0;
    pub const B: u16 = 1 << 1;
    pub const X: u16 = 1 << 2;
    pub const Y: u16 = 1 << 3;

    // D-pad
    pub const UP:    u16 = 1 << 4;
    pub const DOWN:  u16 = 1 << 5;
    pub const LEFT:  u16 = 1 << 6;
    pub const RIGHT: u16 = 1 << 7;

    // Triggers
    pub const L1: u16 = 1 << 8;
    pub const L2: u16 = 1 << 9;
    pub const R1: u16 = 1 << 10;
    pub const R2: u16 = 1 << 11;

    // Stick clicks
    pub const L3: u16 = 1 << 12;
    pub const R3: u16 = 1 << 13;

    // Menu
    pub const SELECT: u16 = 1 << 14;
    pub const START:  u16 = 1 << 15;
}

/// Hat switch (D-pad) values
#[repr(u8)]
pub enum DPad {
  North     = 0,
  NorthEast = 1,
  East      = 2,
  SouthEast = 3,
  South     = 4,
  SouthWest = 5,
  West      = 6,
  NorthWest = 7,
  Centered  = 8,
}

#[derive(IntoBytes, Immutable)]
#[repr(packed)]
struct GamepadReport {
  buttons: u16, // 16 buttons
  lx: i8,       // Left stick X  (-127 to 127)
  ly: i8,       // Left stick Y  (-127 to 127)
  rx: i8,       // Right stick X (-127 to 127)
  ry: i8,       // Right stick Y (-127 to 127)
}

struct Gamepad {
  server: &'static mut BLEServer,
  input_gamepad: Arc<Mutex<BLECharacteristic>>,
  report: GamepadReport,
}

impl Gamepad {
  fn new() -> anyhow::Result<Self> {
    let device = BLEDevice::take();
    
    device
  .security()
  .set_auth(AuthReq::Bond)
  .set_io_cap(SecurityIOCap::NoInputNoOutput)
  .set_passkey(0)
  .resolve_rpa();

    let server = device.get_server();

    server.on_connect(|server, desc| {
        log::info!("Client connected: {:?}", desc);
        server.update_conn_params(desc.conn_handle(), 16, 32, 0, 600).unwrap();
    });

    server.on_disconnect(|_desc, reason| {
        log::info!("Client disconnected, reason: {:?}", reason);
        BLEDevice::take().get_advertising().lock().start().unwrap();
    });

    
    let mut hid = BLEHIDDevice::new(server);

    let input_gamepad = hid.input_report(GAMEPAD_ID);

    hid.manufacturer("Espressif");
    hid.pnp(0x02, 0x045e, 0x028e, 0x0114); // Xbox 360 controller PnP IDs
    hid.hid_info(0x00, 0x01);
    hid.report_map(HID_REPORT_DESCRIPTOR);
    hid.set_battery_level(100);

    let ble_advertising = device.get_advertising();
    ble_advertising.lock().scan_response(false).set_data(
      BLEAdvertisementData::new()
        .name("ESP32 Gamepad")
        .appearance(0x03C4)
        .add_service_uuid(hid.hid_service().lock().uuid()),
    )?;
    ble_advertising.lock().start()?;
    Ok(Self {
      server,
      input_gamepad,
      report: GamepadReport {
        buttons: 0,
        lx: 0,
        ly: 0,
        rx: 0,
        ry: 0,
      },
    })
  }

  fn connected(&self) -> bool {
    self.server.connected_count() > 0
  }

  fn send_report(&self) {
    self
      .input_gamepad
      .lock()
      .set_value(self.report.as_bytes())
      .notify();
    esp_idf_svc::hal::delay::Ets::delay_ms(7);
  }

  fn press(&mut self, button: u16) {
    self.report.buttons |= button;
    self.send_report();
  }

  fn release(&mut self, button: u16) {
    self.report.buttons &= !button;
    self.send_report();
  }

  fn release_all(&mut self) {
    self.report.buttons = 0;
    self.send_report();
  }

  fn set_left_stick(&mut self, x: i8, y: i8) {
    self.report.lx = x;
    self.report.ly = y;
    self.send_report();
  }

  fn set_right_stick(&mut self, x: i8, y: i8) {
    self.report.rx = x;
    self.report.ry = y;
    self.send_report();
  }

}

pub fn start(display: &mut OledDisplay, buttons: &Buttons, joysticks: &mut Joysticks)  {
    log::info!("Gamepad initialized, waiting for connection...");
    display.clear();
    let mut gamepad = match Gamepad::new() {
     Ok(g) => g,
     Err(e) => {
         log::error!("Gamepad::new() failed: {:?}", e);
         return;
     }
    };
     let menu_style = MonoTextStyleBuilder::new()
        .font(&FONT_4X6)
        .text_color(BinaryColor::On)
        .build();    
      Text::new(
            "playing on a bluetooth device",
            Point::new(20,50),
            menu_style
        ).draw(display).unwrap();
      display.flush(); 
    loop {
        if gamepad.connected() {
            let b = buttons.read();
            let left_stick = joysticks.read_left();
            let right_stick = joysticks.read_right();

            let mut btn: u16 = 0;

            // Action
            if b.a { btn |= Button::A; }
            if b.b { btn |= Button::B; }
            if b.x { btn |= Button::X; }
            if b.y { btn |= Button::Y; }

            // D-pad
            if b.up    { btn |= Button::UP; }
            if b.down  { btn |= Button::DOWN; }
            if b.left  { btn |= Button::LEFT; }
            if b.right { btn |= Button::RIGHT; }

            // Triggers
            if b.l1 { btn |= Button::L1; }
            if b.l2 { btn |= Button::L2; }
            if b.r1 { btn |= Button::R1; }
            if b.r2 { btn |= Button::R2; }

            // Stick clicks
            if b.l3 { btn |= Button::L3; }
            if b.r3 { btn |= Button::R3; }

            // Menu
            if b.select { btn |= Button::SELECT; }
            if b.start  { btn |= Button::START; }

            gamepad.report.buttons = btn;
            gamepad.report.lx = left_stick.x;
            gamepad.report.ly = left_stick.y;
            gamepad.report.rx = right_stick.x;
            gamepad.report.ry = right_stick.y;
            gamepad.send_report();
        }

        FreeRtos::delay_ms(10);
    }
}