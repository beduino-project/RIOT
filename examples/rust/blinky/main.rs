#![feature(lang_items)]
#![no_main]
#![no_std]

mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}

extern crate gpio;
extern crate xtimer;

use gpio::Pin;
use xtimer::Duration::Seconds;

#[no_mangle]
pub fn main() {
    let pin = Pin::from_tuple(2, 13);
    loop {
        xtimer::sleep(Seconds(1));
        pin.toggle_value();
    }
}
