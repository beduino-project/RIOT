//! # IP address module.
//!
//! This is very closely modelled after `std::net::IpAddr`.

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

    /// Returns the four 8-bit segments which make up this address.
    pub fn segments(&self) -> [u8; 4] {
        self.0
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

    /// Returns the sixteen 8-bit segments which make up this address.
    pub fn segments(&self) -> [u8; 16] {
        self.0
    }
}
