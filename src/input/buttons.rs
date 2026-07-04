use esp_idf_svc::hal::{
    delay::FreeRtos,
    gpio::{AnyIOPin, AnyOutputPin, Input, Output, PinDriver, Pull},
};
use embedded_hal::delay::DelayNs;

const ROWS: usize = 4;
const COLS: usize = 4;

//         COL0    COL1    COL2    COL3
// ROW0  [ X    ][ Y    ][ A    ][ B    ]
// ROW1  [ UP   ][ DOWN ][ LEFT ][ RIGHT]
// ROW2  [ L1   ][ L2   ][ R1   ][ R2   ]
// ROW3  [ L3   ][ R3   ][ START][ SEL  ]

pub struct Buttons<'a> {
    rows: [PinDriver<'a, AnyOutputPin, Output>; ROWS],
    cols: [PinDriver<'a, AnyIOPin, Input>; COLS],
    back: PinDriver<'a, AnyIOPin, Input>,
}

#[derive(Debug, Default, Clone)]
pub struct ButtonState {
    pub x: bool, pub y: bool, pub a: bool, pub b: bool,
    pub up: bool, pub down: bool, pub left: bool, pub right: bool,
    pub l1: bool, pub l2: bool, pub r1: bool, pub r2: bool,
    pub l3: bool, pub r3: bool, pub start: bool, pub select: bool,
    pub back: bool,
}

impl<'a> Buttons<'a> {
    pub fn new(
        row0: AnyOutputPin, row1: AnyOutputPin,
        row2: AnyOutputPin, row3: AnyOutputPin,
        col0: AnyIOPin, col1: AnyIOPin,
        col2: AnyIOPin, col3: AnyIOPin,
        pin_back: AnyIOPin,
    ) -> Self {
        let mut make_row = |pin: AnyOutputPin| {
            let mut p = PinDriver::output(pin).unwrap();
            p.set_high().unwrap(); // idle HIGH
            p
        };
        let mut make_col = |pin: AnyIOPin| {
            let mut p = PinDriver::input(pin).unwrap();
            p.set_pull(Pull::Up).unwrap(); // idle HIGH via pull-up
            p
        };
        let mut back = PinDriver::input(pin_back).unwrap();
        back.set_pull(Pull::Up).unwrap();

        Self {
            rows: [make_row(row0), make_row(row1), make_row(row2), make_row(row3)],
            cols: [make_col(col0), make_col(col1), make_col(col2), make_col(col3)],
            back,
        }
    }

    pub fn read(&mut self) -> ButtonState {
        // We scan one row at a time:
        //   1. pull that row LOW
        //   2. check each column — if a button is pressed, that col will also read LOW
        //   3. put the row back HIGH before moving to the next row

        let mut pressed = [[false; COLS]; ROWS];

        for row in 0..ROWS {
            self.rows[row].set_low().unwrap();  // activate this row
            FreeRtos.delay_us(10);              // wait for the pin to settle

            for col in 0..COLS {
                pressed[row][col] = self.cols[col].is_low(); // LOW = button pressed
            }

            self.rows[row].set_high().unwrap(); // deactivate before next row
        }

        ButtonState {
            x:      pressed[0][0],
            y:      pressed[0][1],
            a:      pressed[0][2],
            b:      pressed[0][3],
            up:     pressed[1][0],
            down:   pressed[1][1],
            left:   pressed[1][2],
            right:  pressed[1][3],
            l1:     pressed[2][0],
            l2:     pressed[2][1],
            r1:     pressed[2][2],
            r2:     pressed[2][3],
            l3:     pressed[3][0],
            r3:     pressed[3][1],
            start:  pressed[3][2],
            select: pressed[3][3],
            back:   self.back.is_low(),
        }
    }
    pub fn debug_scan(&mut self) {
    loop {
        // print raw col states with NO row driven (all rows HIGH)
        log::info!(
            "IDLE cols: {} {} {} {}",
            self.cols[0].is_low(),
            self.cols[1].is_low(),
            self.cols[2].is_low(),
            self.cols[3].is_low(),
        );

        // now drive each row LOW one at a time and print cols
        for row in 0..ROWS {
            self.rows[row].set_low().unwrap();
            FreeRtos.delay_us(50);
            log::info!(
                "ROW{} LOW -> cols: {} {} {} {}",
                row,
                self.cols[0].is_low(),
                self.cols[1].is_low(),
                self.cols[2].is_low(),
                self.cols[3].is_low(),
            );
            self.rows[row].set_high().unwrap();
        }

        FreeRtos::delay_ms(500);
    }
}
}