#![no_main]
#![no_std]

extern crate kernel;
extern crate periph;
extern crate xtimer;

use periph::gpio::Pin;
use xtimer::Duration::Seconds;

#[no_mangle]
pub fn main() {
    let pin = Pin::from_tuple(2, 13);
    loop {
        xtimer::sleep(Seconds(1));
        pin.toggle_value();
    }
}
