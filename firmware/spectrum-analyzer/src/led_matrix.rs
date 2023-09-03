use attiny_hal as hal;
use hal::port::{mode::Output, Pin, PB0, PB1, PB2};

use crate::delay::delay_ms;

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

    pub fn display(&mut self, data: &[[bool; 7]; 20]) {
        //
    }
}
