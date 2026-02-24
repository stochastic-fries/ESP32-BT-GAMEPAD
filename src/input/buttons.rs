use esp_idf_svc::hal::gpio::{AnyInputPin, Input, PinDriver, Pull,AnyIOPin};

pub struct Buttons<'a>{
    pub x:     PinDriver<'a, AnyIOPin, Input>,
    pub y:     PinDriver<'a, AnyIOPin, Input>,
    pub a:     PinDriver<'a, AnyIOPin, Input>,
    pub b:     PinDriver<'a, AnyIOPin, Input>,

    pub up:    PinDriver<'a, AnyIOPin, Input>,
    pub down:  PinDriver<'a, AnyIOPin, Input>,
    pub left:  PinDriver<'a, AnyIOPin, Input>,
    pub right: PinDriver<'a, AnyIOPin, Input>,

    pub l1:     PinDriver<'a, AnyIOPin, Input>,
    pub l2:     PinDriver<'a, AnyIOPin, Input>,

    pub r1:     PinDriver<'a, AnyIOPin, Input>,
    pub r2:     PinDriver<'a, AnyIOPin, Input>,
    
    pub start:     PinDriver<'a, AnyIOPin, Input>,
    pub select:     PinDriver<'a, AnyIOPin, Input>,
    pub back:     PinDriver<'a, AnyIOPin, Input>,
    

}

#[derive(Debug, Default, Clone)]
pub struct ButtonState {
    pub x: bool,
    pub y: bool,
    pub a: bool,
    pub b: bool,

    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,

    pub l1 : bool,
    pub l2 : bool,
    pub l3 : bool,
    pub r1 : bool,
    pub r2 : bool,
    pub r3 : bool,

    pub start: bool,
    pub select: bool,
    pub back: bool,

}
impl<'a> Buttons<'a> {

    // Call this once in main.rs to set up all buttons
    // while calling this func. order matters
    pub fn new(
        pin_x:     AnyIOPin,
        pin_y:     AnyIOPin,
        pin_a:     AnyIOPin,
        pin_b:     AnyIOPin,
        
        pin_up:    AnyIOPin,
        pin_down:  AnyIOPin,
        pin_left:  AnyIOPin,
        pin_right: AnyIOPin,
    
        pin_l1: AnyIOPin,
        pin_l2: AnyIOPin,

        pin_r1: AnyIOPin,
        pin_r2: AnyIOPin,

        pin_start: AnyIOPin, 
        pin_select: AnyIOPin,
        pin_back: AnyIOPin,

    ) -> Self {

        // setting up each pin
        let mut setup = |pin: AnyIOPin| {
            let mut p = PinDriver::input(pin).unwrap();
            p.set_pull(Pull::Up).unwrap(); // HIGH = not pressed, LOW = pressed
            p
        };

        Self {
            x: setup(pin_x),
            y:setup(pin_y),
            a:     setup(pin_a),
            b:     setup(pin_b),

            up:    setup(pin_up),
            down:  setup(pin_down),
            left:  setup(pin_left),
            right: setup(pin_right),
        
            start: setup(pin_start),
            select: setup(pin_select),
            back: setup(pin_back),

            l1: setup(pin_l1),
            l2: setup(pin_l2),
            r1: setup(pin_r1),
            r2: setup(pin_r2),
        }
    }

    // Call this every loop tick to get current button states
    pub fn read(&self) -> ButtonState {
        let l1 = self.l1.is_low();
        let l2 = self.l2.is_low();
        let r1 = self.r1.is_low();
        let r2 = self.r2.is_low();

        ButtonState {
            x: self.x.is_low(),
            y: self.y.is_low(),
            a:     self.a.is_low(),
            b:     self.b.is_low(),

            up:    self.up.is_low(),
            down:  self.down.is_low(),
            left:  self.left.is_low(),
            right: self.right.is_low(),
        
            l1,
            l2,
            l3: l1 && l2,
            r1,
            r2,
            r3: r1 && r2,

            start: self.start.is_low(),
            select: self.select.is_low(),
            back: self.back.is_low(),
        }
    }
}
