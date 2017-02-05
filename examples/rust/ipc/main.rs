#![feature(lang_items)]
#![no_main]
#![no_std]

mod lang_items {
    #[lang = "panic_fmt"]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    fn eh_personality() {}
}

extern crate kernel;

use core::mem;

use kernel::msg;
use kernel::msg::msg_t;

#[no_mangle]
pub fn main() {
    let mut queue: [msg_t; 4] = unsafe {
        mem::uninitialized()
    };

    let cont  = msg::content { value: 42 };
    let mut m = msg_t::new(0, cont);

    msg::init_queue(&mut queue);
    msg::send_to_self(&mut m);
    msg::print_queue();
}
