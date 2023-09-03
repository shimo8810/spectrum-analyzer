#![no_std]
#![no_main]

use attiny_hal as hal;
// use panic_halt as _;

use spectrum_analyzer::delay::delay_ms;
use spectrum_analyzer::led_matrix::LedMatrix;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    let data = [
        0b00000000_10000011_10000011_00101111, // L1
        0b00000000_00000001_00000011_00010011, // L2
        0b00000000_00000001_00000111_00000011, // L3
        0b00000000_00000000_00000000_00000000, // L4
        0b00000000_10000011_11000011_00001111, // L5
        0b00000000_00001001_00000011_00000011, // L6
        0b00000000_11000011_10000011_00001111, // L7
        0b00000000_10000011_10000011_01001111, // R1
        0b00000000_00000001_00000011_10000011, // R2
        0b00000000_00000001_00001011_00000011, // R3
        0b00000000_00000000_00000000_00000000, // R4
        0b00000000_10000111_10000011_00001111, // R5
        0b00000000_00010000_00000000_00001111, // R6
        0b00000000_10100011_10000011_00001000, // R7
    ];
    avr_device::interrupt::disable();
    let dp = unsafe { hal::Peripherals::steal() };
    let pins = hal::pins!(dp);

    let mut display = LedMatrix::new(
        pins.pb0.into_output(),
        pins.pb1.into_output(),
        pins.pb2.into_output(),
    );

    display.update_shift_register(0);

    loop {
        for &d in &data {
            display.update_shift_register(d);
            delay_ms(1);
        }
    }
}

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins: hal::Pins = hal::pins!(dp);

    let mut display = LedMatrix::new(
        pins.pb0.into_output(),
        pins.pb1.into_output(),
        pins.pb2.into_output(),
    );

    // let data = [
    //     0b00000000_10000011_10000011_00101111,
    //     0b00000000_10000011_10000011_00011111,
    //     0b00000000_10000011_10000111_00001111,
    //     0b00000000_10000011_10010011_00001111,
    //     0b00000000_10000011_11000011_00001111,
    //     0b00000000_10001011_10000011_00001111,
    //     0b00000000_11000011_10000011_00001111,
    //     0b00000000_10000011_10000011_01001111,
    //     0b00000000_10000011_10000011_10001111,
    //     0b00000000_10000011_10001011_00001111,
    //     0b00000000_10000011_10100011_00001111,
    //     0b00000000_10000111_10000011_00001111,
    //     0b00000000_10010011_10000011_00001111,
    //     0b00000000_10100011_10000011_00001111,
    // ];
    // let data = [
    //     0bx00000xx_x00000xx_0010xxxx, 5  L1
    //     0bx00000xx_x00000xx_0001xxxx, 4  L2
    //     0bx00000xx_x00001xx_0000xxxx, 10 L3
    //     0bx00000xx_x00100xx_0000xxxx, 12 L4
    //     0bx00000xx_x10000xx_0000xxxx, 14 L5
    //     0bx00010xx_x00000xx_0000xxxx, 19 L6
    //     0bx10000xx_x00000xx_0000xxxx, 22 L7
    //     0bx00000xx_x00000xx_0100xxxx, 6  R1
    //     0bx00000xx_x00000xx_1000xxxx, 7  R2
    //     0bx00000xx_x00010xx_0000xxxx, 11 R3
    //     0bx00000xx_x01000xx_0000xxxx, 13 R4
    //     0bx00001xx_x00000xx_0000xxxx, 18 R5
    //     0bx00100xx_x00000xx_0000xxxx, 20 R6
    //     0bx01000xx_x00000xx_0000xxxx, 21 R7
    // ];

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
    // led.update_shift_register(0);
    delay_ms(500);

    let data3 = [[true; 7]; 20];

    let ddd = [
        [true, false, true, false, true, false, true],
        [false, true, false, true, false, true, false],
        [false, false, false, false, false, false, false],
        [true, false, true, false, true, false, true],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [true, true, true, true, true, true, true],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [true, false, true, false, true, false, true],
        [false, false, false, false, false, false, false],
        [false, true, false, true, false, true, false],
        [true, false, true, false, true, false, true],
        [false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false],
        [true, false, true, false, true, false, true],
        [false, true, false, true, false, true, false],
        [false, false, false, false, false, false, false],
        [true, false, true, false, true, false, true],
        [false, false, false, false, false, false, false],
    ];
    for _ in 0..50 {
        for &d in &data {
            display.update_shift_register(d);
            delay_ms(1);
        }
    }
    loop {
        display.show(&ddd);
    }
}
