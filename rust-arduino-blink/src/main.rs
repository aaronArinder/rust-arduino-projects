// in an embedded env, no standard library
#![no_std]
// defining a different entrypoint than main
#![no_main]

// in programs not using standard library, panic is undefined behavior; this
// crate defines it for us
extern crate panic_halt;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;

// entrypoint defined
#[arduino_uno::entry]
fn main() -> ! {
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    // built-in led
    let mut led = pins.d13.into_output(&mut pins.ddr);

    loop {
        stutter_blink(&mut led, 25);
    }
}

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}