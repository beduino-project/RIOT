//! # Sock address module.
//!
//! This is very closely modelled after `std::net::SocketAddr`.
//! It is mapped to the `_sock_tl_ep` RIOT type from sock.

use ::ipaddr::IpAddr;
use ::ipaddr::Ipv4Addr;
use ::ipaddr::Ipv6Addr;

use ::ffi::sock_udp_ep_t;
use ::ffi::_sock_tl_ep as sock_ep;
use ::ffi::_sock_tl_ep__bindgen_ty_1
    as sock_ep_union;

use cpu::libc::c_int;

use core::fmt;

extern {
    static XAF_INET: c_int;
    static XAF_INET6: c_int;
    static XSOCK_ADDR_ANY_NETIF: u16;
}

/// Representation of a socket address for networking applications.
/// Either using an IPv4 or an IPv6 address.
pub enum SocketAddr {
    V4(SocketAddrV4),
    V6(SocketAddrV6),
}

/// An IPv4 socket address which is a combination of an IPv4 address and
/// a 16-bit port number.
pub struct SocketAddrV4(sock_ep);

/// An IPv6 socket address which is a combination of an IPv6 address and
/// a 16-bit port number.
pub struct SocketAddrV6(sock_ep);

impl SocketAddr {
    /// Creates a new SocketAddr from an ip address and a 16-bit port.
    pub fn new(a: IpAddr, p: u16) -> Self {
        match a {
            IpAddr::V4(a) => SocketAddr::V4(SocketAddrV4::new(a, p)),
            IpAddr::V6(a) => SocketAddr::V6(SocketAddrV6::new(a, p)),
        }
    }

    /// Creates a new SocketAddr from a `_sock_tl_ep`.
    pub unsafe fn from_sock_ep(e: sock_ep) -> Self {
        if e.family == XAF_INET {
            SocketAddr::V4(SocketAddrV4(e))
        } else {
            SocketAddr::V6(SocketAddrV6(e))
        }
    }

    /// Returns a representation of this address as a UDP endpoint.
    pub fn udp_ep(self) -> sock_udp_ep_t {
        match self {
            SocketAddr::V4(a) => a.0 as sock_udp_ep_t,
            SocketAddr::V6(a) => a.0 as sock_udp_ep_t,
        }
    }
}

impl fmt::Display for SocketAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &SocketAddr::V4(ref a) => write!(f, "{}", a),
            &SocketAddr::V6(ref a) => write!(f, "{}", a),
        }
    }
}

impl SocketAddrV4 {
    /// Create a new socket address from an IPv4 address and a 16-bit
    /// port.
    pub fn new(a: Ipv4Addr, p: u16) -> Self {
        let i = unsafe {
            sock_ep {
                family: XAF_INET,
                addr: sock_ep_union {
                    ipv4: a.segments(),
                },
                port: p,
                netif: XSOCK_ADDR_ANY_NETIF, // TODO
            }
        };

        SocketAddrV4(i)
    }
}

impl fmt::Display for SocketAddrV4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = unsafe {
            Ipv4Addr::from_segments(self.0.addr.ipv4)
        };

        write!(f, "{}:{}", addr, self.0.port)
    }
}

impl SocketAddrV6 {
    /// Create a new socket address from an IPv6 address and a 16-bit
    /// port.
    pub fn new(a: Ipv6Addr, p: u16) -> Self {
        let i = unsafe {
            sock_ep {
                family: XAF_INET6,
                addr: sock_ep_union {
                    ipv6: a.segments(),
                },
                port: p,
                netif: XSOCK_ADDR_ANY_NETIF, // TODO
            }
        };

        SocketAddrV6(i)
    }
}

impl fmt::Display for SocketAddrV6 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = unsafe {
            Ipv6Addr::from_segments(self.0.addr.ipv6)
        };

        write!(f, "[{}]:{}", addr, self.0.port)
    }
}
