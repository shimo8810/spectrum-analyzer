use attiny_hal as hal;
use hal::port::{mode::Output, Pin, PB0, PB1, PB2};

use crate::delay::delay_ms;

const NUM_COL: i8 = 7;
const NUM_ROW: i8 = 20;
enum Index {
    Row(i8),
    Rcol(i8),
    Lcol(i8),
}

const INDICES: [Index; 24] = [
    Index::Row(0),  // LED A
    Index::Row(1),  // LED B
    Index::Row(2),  // LED C
    Index::Row(3),  // LED D
    Index::Rcol(1), // RIGHT 2
    Index::Rcol(0), // RIGHT 1
    Index::Lcol(0), // LEFT 1
    Index::Lcol(1), // LEFT 2
    Index::Row(8),  // LED I
    Index::Row(9),  // LED J
    Index::Rcol(2), // RIGHT 3
    Index::Lcol(2), // LEFT 3
    Index::Rcol(3), // RIGHT 4
    Index::Lcol(3), // LEFT 4
    Index::Rcol(4), // RIGHT 5
    Index::Row(4),  // LED E
    Index::Row(7),  // LED H
    Index::Row(6),  // LED G
    Index::Lcol(4), // LEFT 5
    Index::Rcol(5), // RIGHT 6
    Index::Lcol(5), // LEFT 6
    Index::Lcol(6), // LEFT 7
    Index::Rcol(6), // RIGHT 7
    Index::Row(5),  // LED F
];
pub struct LedMatrix {
    data_pin: Pin<Output, PB0>,
    latch_pin: Pin<Output, PB1>,
    clock_pin: Pin<Output, PB2>,
}

impl LedMatrix {
    pub fn new(
        data_pin: Pin<Output, PB0>,
        latch_pin: Pin<Output, PB1>,
        clock_pin: Pin<Output, PB2>,
    ) -> Self {
        Self {
            data_pin,
            latch_pin,
            clock_pin,
        }
    }

    pub fn shift_out(&mut self, data: u32) {
        for i in 0..24 {
            if data & (1 << i) == 0 {
                self.data_pin.set_low();
            } else {
                self.data_pin.set_high();
            }
            self.clock_pin.set_high();
            self.clock_pin.set_low();
        }
    }

    pub fn update_shift_register(&mut self, data: u32) {
        self.latch_pin.set_low();

        self.shift_out(data);

        self.latch_pin.set_high();
    }

    pub fn show(&mut self, data: &[[bool; NUM_COL as usize]; NUM_ROW as usize]) {
        for n in 0i8..(NUM_COL * 2) {
            self.latch_pin.set_low();
            for i in &INDICES {
                if match i {
                    Index::Row(k) => {
                        data[((NUM_ROW / 2 - 1) + (2 * (n / NUM_COL) - 1) * k + n / NUM_COL)
                            as usize][(n % NUM_COL) as usize]
                    }
                    Index::Rcol(k) => n / NUM_COL == 0 && n % NUM_COL == *k,
                    Index::Lcol(k) => n / NUM_COL == 1 && n % NUM_COL == *k,
                } {
                    self.data_pin.set_high();
                } else {
                    self.data_pin.set_low();
                }
                self.clock_pin.set_high();
                self.clock_pin.set_low();
            }
            self.latch_pin.set_high();

            delay_ms(1);
        }
    }
}
