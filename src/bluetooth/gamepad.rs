// originally: https://github.com/T-vK/ESP32-BLE-Keyboard
// the keyboard HID code is taken from a github page , from the example section and give to claude
// and claude re wrote it for gamepad 
#![allow(dead_code)]

use esp32_nimble::{
  BLEAdvertisementData, BLECharacteristic, BLEDevice, BLEHIDDevice, BLEServer, enums::*, hid::*,
  utilities::mutex::Mutex,
};
use std::sync::Arc;
use zerocopy::IntoBytes;
use zerocopy_derive::{Immutable, IntoBytes};

const GAMEPAD_ID: u8 = 0x01;

const HID_REPORT_DESCRIPTOR: &[u8] = hid!(
  (USAGE_PAGE, 0x01),       // USAGE_PAGE (Generic Desktop)
  (USAGE, 0x05),            // USAGE (Gamepad)
  (COLLECTION, 0x01),       // COLLECTION (Application)
  (REPORT_ID, GAMEPAD_ID),  //   REPORT_ID (1)
  // ------------------------------------------------- Buttons (16 buttons)
  (USAGE_PAGE, 0x09),       //   USAGE_PAGE (Button)
  (USAGE_MINIMUM, 0x01),    //   USAGE_MINIMUM (Button 1)
  (USAGE_MAXIMUM, 0x10),    //   USAGE_MAXIMUM (Button 16)
  (LOGICAL_MINIMUM, 0x00),  //   LOGICAL_MINIMUM (0)
  (LOGICAL_MAXIMUM, 0x01),  //   LOGICAL_MAXIMUM (1)
  (REPORT_SIZE, 0x01),      //   REPORT_SIZE (1)
  (REPORT_COUNT, 0x10),     //   REPORT_COUNT (16)
  (HIDINPUT, 0x02),         //   INPUT (Data,Var,Abs)
  // ------------------------------------------------- Axes (4 axes: LX, LY, RX, RY)
  (USAGE_PAGE, 0x01),       //   USAGE_PAGE (Generic Desktop)
  (USAGE, 0x30),            //   USAGE (X)
  (USAGE, 0x31),            //   USAGE (Y)
  (USAGE, 0x32),            //   USAGE (Z)
  (USAGE, 0x35),            //   USAGE (Rz)
  (LOGICAL_MINIMUM, 0x81),  //   LOGICAL_MINIMUM (-127)
  (LOGICAL_MAXIMUM, 0x7F),  //   LOGICAL_MAXIMUM (127)
  (REPORT_SIZE, 0x08),      //   REPORT_SIZE (8)
  (REPORT_COUNT, 0x04),     //   REPORT_COUNT (4)
  (HIDINPUT, 0x02),         //   INPUT (Data,Var,Abs)
  // ------------------------------------------------- Hat switch (D-pad)
  (USAGE_PAGE, 0x01),       //   USAGE_PAGE (Generic Desktop)
  (USAGE, 0x39),            //   USAGE (Hat switch)
  (LOGICAL_MINIMUM, 0x00),  //   LOGICAL_MINIMUM (0)
  (LOGICAL_MAXIMUM, 0x07),  //   LOGICAL_MAXIMUM (7)
  (PHYSICAL_MINIMUM, 0x00), //   PHYSICAL_MINIMUM (0)
  (PHYSICAL_MAXIMUM, 0x3B, 0x01), // PHYSICAL_MAXIMUM (315)
  (UNIT, 0x14),             //   UNIT (Eng Rot: Degree)
  (REPORT_SIZE, 0x04),      //   REPORT_SIZE (4)
  (REPORT_COUNT, 0x01),     //   REPORT_COUNT (1)
  (HIDINPUT, 0x42),         //   INPUT (Data,Var,Abs,Null)
  // ------------------------------------------------- Padding
  (REPORT_SIZE, 0x04),      //   REPORT_SIZE (4)
  (REPORT_COUNT, 0x01),     //   REPORT_COUNT (1)
  (HIDINPUT, 0x01),         //   INPUT (Const,Array,Abs)
  (END_COLLECTION),         // END_COLLECTION
);

/// Gamepad buttons bitmask constants
pub struct Button;
impl Button {
  pub const CROSS:     u16 = 1 << 0;
  pub const CIRCLE:    u16 = 1 << 1;
  pub const SQUARE:    u16 = 1 << 2;
  pub const TRIANGLE:  u16 = 1 << 3;
  pub const L1:        u16 = 1 << 4;
  pub const R1:        u16 = 1 << 5;
  pub const L2:        u16 = 1 << 6;
  pub const R2:        u16 = 1 << 7;
  pub const SELECT:    u16 = 1 << 8;
  pub const START:     u16 = 1 << 9;
  pub const L3:        u16 = 1 << 10;
  pub const R3:        u16 = 1 << 11;
  pub const HOME:      u16 = 1 << 12;
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
  hat: u8,      // D-pad hat switch (upper nibble = padding, lower nibble = direction)
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
      .set_auth(AuthReq::all())
      .set_io_cap(SecurityIOCap::NoInputNoOutput)
      .resolve_rpa();

    let server = device.get_server();
    let mut hid = BLEHIDDevice::new(server);

    let input_gamepad = hid.input_report(GAMEPAD_ID);

    hid.manufacturer("Espressif");
    hid.pnp(0x02, 0x05ac, 0x820a, 0x0210);
    hid.hid_info(0x00, 0x01);

    hid.report_map(HID_REPORT_DESCRIPTOR);

    hid.set_battery_level(100);

    let ble_advertising = device.get_advertising();
    ble_advertising.lock().scan_response(false).set_data(
      BLEAdvertisementData::new()
        .name("ESP32 Gamepad")
        .appearance(0x03C4) // HID Gamepad appearance
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
        hat: DPad::Centered as u8,
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

  fn set_dpad(&mut self, direction: DPad) {
    self.report.hat = direction as u8;
    self.send_report();
  }
}

pub fn start() -> anyhow::Result<()> {
  esp_idf_svc::sys::link_patches();
  esp_idf_svc::log::EspLogger::initialize_default();

  log::info!("Starting BLE gamepad...");

  let mut gamepad = match Gamepad::new() {
    Ok(g) => {
      log::info!("Gamepad initialized successfully");
      g
    }
    Err(e) => {
      log::error!("Failed to initialize gamepad: {:?}", e);
      return Err(e);
    }
  };

  log::info!("Entering main loop");
  loop {
    if gamepad.connected() {
      log::info!("Pressing Cross button...");
      gamepad.press(Button::CROSS);
      esp_idf_svc::hal::delay::FreeRtos::delay_ms(500);

      gamepad.release(Button::CROSS);
      esp_idf_svc::hal::delay::FreeRtos::delay_ms(500);

      log::info!("Moving left stick...");
      gamepad.set_left_stick(127, 0);
      esp_idf_svc::hal::delay::FreeRtos::delay_ms(500);

      gamepad.set_left_stick(0, 0);
      esp_idf_svc::hal::delay::FreeRtos::delay_ms(500);
    } else {
      log::info!("Waiting for connection...");
      esp_idf_svc::hal::delay::FreeRtos::delay_ms(500);
    }
  }
}