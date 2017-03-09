#![feature(lang_items)]
#![allow(private_no_mangle_fns)]
#![no_main]
#![no_std]

mod lang_items {
    #[lang = "panic_fmt"]
    #[no_mangle]
    extern fn panic_fmt() {}

    #[lang = "eh_personality"]
    #[no_mangle]
    fn eh_personality() {}
}

extern crate fmt;
extern crate net;

use fmt::Stdout;
use core::fmt::Write;
use net::ipaddr::Ipv4Addr;
use net::ipaddr::Ipv6Addr;

#[no_mangle]
pub fn main() {
    let mut out = Stdout {};
    let v4 = Ipv4Addr::new(134, 102, 20, 20);
    out.write_fmt(format_args!("The DNS-Server of uni-bremen.de is at {}\n", v4)).unwrap();
    let v6 = Ipv6Addr::new(0x2001, 0x638, 0x708, 0x30da, 0x21e, 0x68ff, 0xfe37, 0xfc76);
    out.write_fmt(format_args!("The one from the tzi is at {}\n", v6)).unwrap();
}
