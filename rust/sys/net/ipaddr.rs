//! # IP address module.
//!
//! This is very closely modelled after `std::net::IpAddr`.

extern crate cpu;

use core::fmt;
use cpu::libc::c_char;

use ::ffi::ipv4_addr_to_str;
use ::ffi::ipv4_addr_t;

use ::ffi::ipv6_addr_to_str;
use ::ffi::ipv6_addr_t;

/// An IP address, either an IPv4 or IPv6 address.
pub enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

/// Struct representing an IPv4 address. This is can be mapped to an
/// array containing 4 u8s, which is the representation used by RIOT.
pub struct Ipv4Addr([u8; 4]);

/// Struct representing an IPv6 address. This is can be mapped to an
/// array containing 16 u8s, which is the representation used by RIOT.
pub struct Ipv6Addr([u8; 16]);

impl Ipv4Addr {
    /// Creates a new IPv4 address from four eight-bit octets.
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Ipv4Addr([a, b, c, d])
    }

    /// Creates a new Ipv4Addr from a an array.
    pub fn from_segments(s: [u8; 4]) -> Self {
        Ipv4Addr(s)
    }

    /// Returns the four 8-bit segments which make up this address.
    pub fn segments(&self) -> [u8; 4] {
        self.0
    }
}

impl fmt::Display for Ipv4Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer: [u8; 20] = [0; 20];
        let v4addr = ipv4_addr_t {
            u8: self.segments()
        };

        let ret = unsafe {
            ipv4_addr_to_str(buffer.as_mut_ptr() as *mut c_char,
                &v4addr, buffer.len() as u8)
        };

        if ret.is_null() {
            return Result::Err(fmt::Error)
        }

        for i in buffer.iter() {
            try!(write!(f, "{}", *i as char));
        }

        Result::Ok(())
    }
}


impl Ipv6Addr {
    /// Creates a new IPv6 address from eight 16-bit segments.
    pub fn new(a: u16, b: u16, c: u16, d: u16,
               e: u16, f: u16, g: u16, h: u16) -> Self {
        Ipv6Addr([(a / 256) as u8, a as u8,
                  (b / 256) as u8, b as u8,
                  (c / 256) as u8, c as u8,
                  (d / 256) as u8, d as u8,
                  (e / 256) as u8, e as u8,
                  (f / 256) as u8, f as u8,
                  (g / 256) as u8, g as u8,
                  (h / 256) as u8, h as u8])
    }

    /// Creates a new Ipv6Addr from a an array.
    pub fn from_segments(s: [u8; 16]) -> Self {
        Ipv6Addr(s)
    }

    /// Returns the sixteen 8-bit segments which make up this address.
    pub fn segments(&self) -> [u8; 16] {
        self.0
    }
}

impl fmt::Display for Ipv6Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buffer: [u8; 39] = [0; 39];
        let v6addr = ipv6_addr_t {
            u8: self.segments()
        };

        let ret = unsafe {
            ipv6_addr_to_str(buffer.as_mut_ptr() as *mut c_char,
                &v6addr, buffer.len() as u8)
        };

        if ret.is_null() {
            return Result::Err(fmt::Error)
        }

        for i in buffer.iter() {
            try!(write!(f, "{}", *i as char));
        }

        Result::Ok(())
    }
}
