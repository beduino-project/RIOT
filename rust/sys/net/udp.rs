//! # UDP module.
//!
//! This module is closely modelled after `std::net::UdpSocket`.

use kernel::error;
use kernel::error::Result;

use core::result::Result::Ok;
use core::result::Result::Err;

use core::ops::Drop;
use core::ptr;
use core::mem;

use cpu::libc::c_void;
use cpu::libc::uint32_t;

use ::sockaddr::SocketAddr;
use ::ffi::sock_udp_ep_t;
use ::ffi::sock_udp_create;
use ::ffi::sock_udp_send;
use ::ffi::sock_udp_recv;
use ::ffi::sock_udp_get_local;
use ::ffi::sock_udp_close;

pub use ::ffi::sock_udp_t;

extern {
    static XSOCK_NO_TIMEOUT: uint32_t;
}

/// A User Datagram Protocol socket.
pub struct UdpSocket<'a>(&'a mut sock_udp_t);

impl<'a> UdpSocket<'a> {
    /// Creates a UDP socket from the given address.
    pub fn bind(s: &'a mut sock_udp_t, a: SocketAddr) -> Result<UdpSocket<'a>> {
        let p = s as *mut sock_udp_t;
        let l = &a.udp_ep() as *const sock_udp_ep_t;

        let ret = unsafe {
            sock_udp_create(p, l, ptr::null(), 0) // TODO flags
        };

        if ret == 0 {
            Ok(UdpSocket(s))
        } else {
            Err(error::decode_errno(-ret))
        }
    }

    /// Sends data on the socket to the given address. On success,
    /// returns the number of bytes written.
    pub fn send_to(&mut self, buf: &[u8], a: SocketAddr) -> Result<isize> {
        let r = &a.udp_ep() as *const sock_udp_ep_t;
        let p = buf.as_ptr() as *const c_void;

        let ret = unsafe {
            sock_udp_send(self.0, p, buf.len(), r)
        };

        if ret >= 0 {
            Ok(ret)
        } else {
            Err(error::decode_errno(-(ret as i32)))
        }
    }

    /// Receives data from the socket. On success, returns the number of
    /// bytes read and the address from whence the data came. Zero is
    /// returned, if no received data is available, but everything is in
    /// order.
    pub fn recv_from(&mut self, buf: &mut [u8]) ->
            Result<(usize, SocketAddr)> {
        let p = buf.as_mut_ptr() as *mut c_void;
        let mut r: sock_udp_ep_t = unsafe {
            mem::uninitialized()
        };

        unsafe {
            let ret = sock_udp_recv(self.0, p, buf.len(),
                XSOCK_NO_TIMEOUT, &mut r);

            if ret >= 0 {
                Ok((ret as usize, SocketAddr::from_sock_ep(r)))
            } else {
                Err(error::decode_errno(-(ret as i32)))
            }
        }
    }

    /// Returns the socket address that this socket was created from.
    pub fn local_addr(&mut self) -> Result<SocketAddr> {
        let mut r: sock_udp_ep_t = unsafe {
            mem::uninitialized()
        };

        unsafe {
            let ret = sock_udp_get_local(self.0, &mut r);
            if ret == 0 {
                Ok(SocketAddr::from_sock_ep(r))
            } else {
                Err(error::decode_errno(-ret))
            }
        }
    }
}

impl<'a> Drop for UdpSocket<'a> {
    fn drop(&mut self) {
        unsafe {
            sock_udp_close(self.0)
        };
    }
}
