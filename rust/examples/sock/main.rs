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

extern crate cpu;
extern crate net;

extern crate xtimer;
extern crate fmt;

use fmt::Stdout;
use core::fmt::Write;
use xtimer::Duration::Seconds;

use cpu::libc::c_int;
use core::ptr;
use core::mem;

use net::sockaddr::SocketAddr;
use net::ipaddr::IpAddr;
use net::ipaddr::Ipv6Addr;

use net::udp::UdpSocket;
use net::udp::sock_udp_t;

extern {
    fn _netif_config(argc: c_int, argv: *mut u8) -> ();
}

#[no_mangle]
pub fn main() {
    let port = 2342;
    let mut out = Stdout {};

    // Wait for autoconfiguration
    writeln!(out, "Waiting for addresses autoconfiguration...\n")
        .unwrap();
    xtimer::sleep(Seconds(3));

    // Print network addresses
    writeln!(out, "Configured network interfaces:").unwrap();
    unsafe {
        _netif_config(0, ptr::null_mut())
    };

    /// FIXME Binds to any IPaddr imho.
    let ipaddr = Ipv6Addr::new(0x2001, 0xdb8, 0, 0, 0, 0, 0, 0x2);
    let soaddr = SocketAddr::new(IpAddr::V6(ipaddr), port);
    let mut socket: sock_udp_t = unsafe {
        mem::uninitialized()
    };

    writeln!(out, "Binding socket to {}", soaddr).unwrap();
    let mut sock = UdpSocket::bind(&mut socket, soaddr)
        .expect("Couldn't bind to address");

    let mut buf = [0; 128];
    loop {
        let (recv, sender) = sock.recv_from(&mut buf).unwrap();
        writeln!(out, "Received {} bytes", recv).unwrap();

        sock.send_to(&buf[0..recv-1], sender).unwrap();
    };
}
