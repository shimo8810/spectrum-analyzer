#![no_std]
#![no_main]

use attiny_hal as hal;
use embedded_hal::blocking::delay::DelayMs;
use hal::port::{mode::Output, Pin, PB0, PB1, PB2};
use panic_halt as _;

pub type Delay = hal::delay::Delay<hal::clock::MHz8>;

pub fn delay_ms(ms: u16) {
    Delay::new().delay_ms(ms);
}

struct LedDisplay {
    data: u32,
    data_pin: Pin<Output, PB0>,
    latch_pin: Pin<Output, PB1>,
    clock_pin: Pin<Output, PB2>,
}

impl LedDisplay {
    pub fn new(
        data_pin: Pin<Output, PB0>,
        latch_pin: Pin<Output, PB1>,
        clock_pin: Pin<Output, PB2>,
    ) -> Self {
        Self {
            data: 0,
            data_pin,
            latch_pin,
            clock_pin,
        }
    }

    fn shift_out(&mut self, data: u32) {
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

    fn update_shift_register(&mut self, data: u32) {
        self.latch_pin.set_low();

        self.shift_out(data);

        self.latch_pin.set_high();
    }
    fn update(&mut self) {
        self.update_shift_register(self.data);
    }
}

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins: hal::Pins = hal::pins!(dp);

    let mut led = LedDisplay::new(
        pins.pb0.into_output(),
        pins.pb1.into_output(),
        pins.pb2.into_output(),
    );

    let data = [
        0b00000000_10000011_10000011_00101111,
        0b00000000_10000011_10000011_00011111,
        0b00000000_10000011_10000111_00001111,
        0b00000000_10000011_10010011_00001111,
        0b00000000_10000011_11000011_00001111,
        0b00000000_10001011_10000011_00001111,
        0b00000000_11000011_10000011_00001111,
        0b00000000_10000011_10000011_01001111,
        0b00000000_10000011_10000011_10001111,
        0b00000000_10000011_10001011_00001111,
        0b00000000_10000011_10100011_00001111,
        0b00000000_10000111_10000011_00001111,
        0b00000000_10010011_10000011_00001111,
        0b00000000_10100011_10000011_00001111,
    ];

    let data = [
        0b00000000_10000011_10000011_00101111, // L1
        0b00000000_00000001_00000011_00010001, // L2
        0b00000000_10000011_10000111_00001111, // L3
        0b00000000_00000000_00000000_00000000, // L4
        0b00000000_10000011_11000011_00001111, // L5
        0b00000000_00001000_00000000_00000001, // L6
        0b00000000_11000011_10000011_00001111, // L7
        0b00000000_10000011_10000011_01001111, // R1
        0b00000000_00000001_00000011_10000001, // R2
        0b00000000_10000011_10001011_00001111, // R3
        0b00000000_00000000_00000000_00000000, // R4
        0b00000000_00000100_00000000_00000001, // R5
        0b00000000_00010000_00000000_00000001, // R6
        0b00000000_10100011_10000011_00001111, // R7
    ];
    let data2 = [
        0b00000000_10000011_10000011_00101111, // L1
        0b00000000_00000001_00000011_00010011, // L2
        0b00000000_10000011_10000111_00001111, // L3
        0b00000000_00000000_00000000_00000000, // L4
        0b00000000_10000011_11000011_00001111, // L5
        0b00000000_10001011_10000011_00001111, // L6
        0b00000000_11000011_10000011_00001111, // L7
        0b00000000_10000011_10000011_01001111, // R1
        0b00000000_00000001_00000011_10000011, // R2
        0b00000000_10000011_10001011_00001111, // R3
        0b00000000_00000000_00000000_00000000, // R4
        0b00000000_10000111_10000011_00001111, // R5
        0b00000000_10010011_10000011_00001111, // R6
        0b00000000_10100011_10000011_00001111, // R7
    ];
    led.update_shift_register(0);
    delay_ms(500);

    loop {
        for _ in 0..100 {
            for &d in &data {
                led.update_shift_register(d);
                delay_ms(0);
            }
        }
        for _ in 0..100 {
            for &d in &data2 {
                led.update_shift_register(d);
                delay_ms(0);
            }
        }
    }
}
