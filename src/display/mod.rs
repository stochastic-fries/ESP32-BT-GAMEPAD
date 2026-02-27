pub mod welcome;
pub mod menu;
pub mod games;
pub mod settings;
use esp_idf_svc::hal::i2c::I2cDriver;
use sh1106::{mode::GraphicsMode, interface::I2cInterface};

pub type OledDisplay<'a> = GraphicsMode<I2cInterface<I2cDriver<'a>>>;