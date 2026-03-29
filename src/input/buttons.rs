use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, AnyOutputPin, Input, Output, PinDriver, Pull},
};

//         COL0    COL1    COL2    COL3
// ROW0  [ X    ][ Y    ][ A    ][ B    ]
// ROW1  [ UP   ][ DOWN ][ LEFT ][ RIGHT]
// ROW2  [ L1   ][ L2   ][ R1   ][ R2   ]
// ROW3  [ L3   ][ R3   ][ START][ SEL  ]
//
// BACK -> direct connection                #not exposed to bluetooth

const ROWS: usize = 4;
const COLS: usize = 4;
const DEBOUNCE_TICKS: u8 = 5;

#[derive(Clone, Copy)]
enum Button {
    X, Y, A, B,
    Up, Down, Left, Right,
    L1, L2, R1, R2,
    L3, R3, Start, Select,
}

#[rustfmt::skip]
const MAP: [[Button; COLS]; ROWS] = [
    [Button::X,   Button::Y,      Button::A,     Button::B     ],
    [Button::Up,  Button::Down,   Button::Left,  Button::Right ],
    [Button::L1,  Button::L2,     Button::R1,    Button::R2    ],
    [Button::L3,  Button::R3,     Button::Start, Button::Select],
];

pub struct Buttons<'a> {
    rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    back: PinDriver<'a, AnyIOPin, Input>,
    debounce: [[u8; COLS]; ROWS],
    back_debounce: u8,
    state: [[bool; COLS]; ROWS],
    back_state: bool,
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
    pub l1: bool,
    pub l2: bool,
    pub l3: bool,
    pub r1: bool,
    pub r2: bool,
    pub r3: bool,
    pub start: bool,
    pub select: bool,
    pub back: bool, // not exposed to bluetooth
}

impl<'a> Buttons<'a> {
    pub fn new(
        // Row pins - outputs, driven LOW during scan
        row0: AnyOutputPin,
        row1: AnyOutputPin,
        row2: AnyOutputPin,
        row3: AnyOutputPin,
        
        // Col pins - inputs with pull-up
        col0: AnyIOPin,
        col1: AnyIOPin,
        col2: AnyIOPin,
        col3: AnyIOPin,

        pin_back: AnyIOPin,
    ) -> Self {
        let mut make_row = |pin: AnyOutputPin| {
            let mut p = PinDriver::output(pin).unwrap();
            p.set_high().unwrap();
            p
        };

        let mut make_col = |pin: AnyIOPin| {
            let mut p = PinDriver::input(pin).unwrap();
            p.set_pull(Pull::Up).unwrap();
            p
        };

        let mut back = PinDriver::input(pin_back).unwrap();
        back.set_pull(Pull::Up).unwrap();

        Self {
            rows: [make_row(row0), make_row(row1), make_row(row2), make_row(row3)],
            cols: [make_col(col0), make_col(col1), make_col(col2), make_col(col3)],
            back,
            debounce: [[0; COLS]; ROWS],
            back_debounce: 0,
            state: [[false; COLS]; ROWS],
            back_state: false,
        }
    }

    // runs one full scan and debounce update. call every 1ms.
    pub fn tick(&mut self) {
        // 4×4 matrix scan 
        for row in 0..ROWS {
            for r in 0..ROWS {
                self.rows[r].set_high().unwrap();
            }
            self.rows[row].set_low().unwrap();


            for col in 0..COLS {
                let pressed = self.cols[col].is_low();

                if pressed == self.state[row][col] {
                    self.debounce[row][col] = 0;
                } else {
                    self.debounce[row][col] += 1;
                    if self.debounce[row][col] >= DEBOUNCE_TICKS {
                        self.state[row][col] = pressed;
                        self.debounce[row][col] = 0;
                    }
                }
            }
        }

        for r in 0..ROWS {
            self.rows[r].set_high().unwrap();
        }

        // --- standalone back pin ---
        let back_pressed = self.back.is_low();
        if back_pressed == self.back_state {
            self.back_debounce = 0;
        } else {
            self.back_debounce += 1;
            if self.back_debounce >= DEBOUNCE_TICKS {
                self.back_state = back_pressed;
                self.back_debounce = 0;
            }
        }
    }

    /// Returns debounced snapshot. Call after tick().
    pub fn read(&self) -> ButtonState {
        let mut out = ButtonState {
            back: self.back_state,
            ..Default::default()
        };

        for row in 0..ROWS {
            for col in 0..COLS {
                let val = self.state[row][col];
                match MAP[row][col] {
                    Button::X      => out.x      = val,
                    Button::Y      => out.y      = val,
                    Button::A      => out.a      = val,
                    Button::B      => out.b      = val,
                    Button::Up     => out.up     = val,
                    Button::Down   => out.down   = val,
                    Button::Left   => out.left   = val,
                    Button::Right  => out.right  = val,
                    Button::L1     => out.l1     = val,
                    Button::L2     => out.l2     = val,
                    Button::R1     => out.r1     = val,
                    Button::R2     => out.r2     = val,
                    Button::L3     => out.l3     = val,
                    Button::R3     => out.r3     = val,
                    Button::Start  => out.start  = val,
                    Button::Select => out.select = val,
                }
            }
        }

        out
    }
}