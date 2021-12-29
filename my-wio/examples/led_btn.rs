#![no_std]
#![no_main]

use panic_halt as _;
use wio_terminal as wio;

use wio::entry;
use wio::hal::gpio::v2::*;
use wio::pac::Peripherals;
use wio::prelude::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let pins = wio::Pins::new(peripherals.PORT);
    let mut led = Led::new(pins.user_led);
    let btn1 = Button1::new(pins.button1);

    loop {
        if btn1.is_pressed() {
            led.turn_on();
        } else {
            led.turn_off();
        }
    }
}

struct Led {
    pin: Pin<PA15, Output<PushPull>>
}

impl Led {
    fn new(pin: Pin<PA15, Disabled<Floating>>) -> Led {
        Led {
            pin: pin.into_push_pull_output()
        }
    }

    fn turn_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn turn_off(&mut self) {
        self.pin.set_low().unwrap();
    }
}

struct Button1 {
    pin: Pin<PC26, Input<Floating>>
}

impl Button1 {
    fn new(pin: Pin<PC26, Disabled<Floating>>) -> Button1 {
        Button1 {
            pin: pin.into_floating_input()
        }
    }

    fn is_pressed(&self) -> bool {
        self.pin.is_low().unwrap()
    }
}