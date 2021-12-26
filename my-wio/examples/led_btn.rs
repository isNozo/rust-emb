#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = wio::Pins::new(peripherals.PORT);
    let mut led = pins.user_led.into_push_pull_output();
    let btn1 = pins.button1.into_floating_input();

    loop {
        if btn1.is_low().unwrap() {
            led.set_high().unwrap();
        } else {
            led.set_low().unwrap();
        }
    }
}
