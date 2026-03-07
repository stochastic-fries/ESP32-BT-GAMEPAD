use std::rc::Rc;
use esp_idf_svc::hal::adc::{
    attenuation::DB_11,
    oneshot::{AdcChannelDriver, AdcDriver, config::AdcChannelConfig},
    ADC1,
};
use esp_idf_svc::hal::gpio::{Gpio32, Gpio33, Gpio34, Gpio35};

#[derive(Debug, Default, Clone)]
pub struct JoystickState {
    pub x: i8,  //as per req. of hid profile
    pub y: i8, 
}

impl JoystickState {
    pub fn is_left(&self)     -> bool { self.x < -50 }
    pub fn is_right(&self)    -> bool { self.x >  50 }
    pub fn is_up(&self)       -> bool { self.y < -50 }
    pub fn is_down(&self)     -> bool { self.y >  50 }
    pub fn is_centered(&self) -> bool { self.x.abs() < 20 && self.y.abs() < 20 }
}

pub struct Joysticks<'a> {
    adc:     Rc<AdcDriver<'a, ADC1>>,
    left_x:  AdcChannelDriver<'a, Gpio34, Rc<AdcDriver<'a, ADC1>>>,
    left_y:  AdcChannelDriver<'a, Gpio35, Rc<AdcDriver<'a, ADC1>>>,
    right_x: AdcChannelDriver<'a, Gpio32, Rc<AdcDriver<'a, ADC1>>>,
    right_y: AdcChannelDriver<'a, Gpio33, Rc<AdcDriver<'a, ADC1>>>,
}

impl<'a> Joysticks<'a> {
    pub fn new(
        adc1:    ADC1,
        left_x:  Gpio34,
        left_y:  Gpio35,
        right_x: Gpio32,
        right_y: Gpio33,
    ) -> Self {
        // Rc lets multiple channel drivers share ownership of the adc driver
        let adc = Rc::new(AdcDriver::new(adc1).unwrap());
        let config = AdcChannelConfig {
            attenuation: DB_11,
            ..Default::default()
        };

        let left_x  = AdcChannelDriver::new(Rc::clone(&adc), left_x,  &config).unwrap();
        let left_y  = AdcChannelDriver::new(Rc::clone(&adc), left_y,  &config).unwrap();
        let right_x = AdcChannelDriver::new(Rc::clone(&adc), right_x, &config).unwrap();
        let right_y = AdcChannelDriver::new(Rc::clone(&adc), right_y, &config).unwrap();

        Self { adc, left_x, left_y, right_x, right_y }
    }

    pub fn read_left(&mut self) -> JoystickState {
        let x = self.adc.read(&mut self.left_x).unwrap_or(2048);
        let y = self.adc.read(&mut self.left_y).unwrap_or(2048);
        JoystickState {
            x: raw_to_axis(x),
            y: raw_to_axis(y),
        }
    }

    pub fn read_right(&mut self) -> JoystickState {
        let x = self.adc.read(&mut self.right_x).unwrap_or(2048);
        let y = self.adc.read(&mut self.right_y).unwrap_or(2048);
        JoystickState {
            x: raw_to_axis(x),
            y: raw_to_axis(y),
        }
    }
}

// tuned based on the observations and may vary depending upon other joysticks

const MIN: i32 = 0;       
const MAX: i32 = 2450;    
const CENTER: i32 = 1075;  
const DEADZONE: i32 = 100;  



fn raw_to_axis(raw: u16) -> i8{

    let raw = raw as i32;
    if (raw - CENTER).abs() <= DEADZONE {
        return 0;
    }

    // maping each side independently so center asymmetry doesn't skew output
    let scaled = if raw > CENTER {
        (raw - CENTER) * 127 / (MAX - CENTER)
    } else {
        (raw - CENTER) * 127 / (CENTER - MIN)
    };

    scaled.clamp(-127, 127) as i8
}
