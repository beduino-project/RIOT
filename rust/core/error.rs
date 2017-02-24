//! # IO Error.
//!
//! Simplified version of `std::io::Error`.

use core::result;
use cpu::libc;

/// A specialized Result type for I/O operations.
/// This type is similar to `std::io::Result`.
pub type Result<T> = result::Result<T, IoError>;

/// A list specifying general categories of I/O error.
/// This is very similar to `std::io::ErrorKind`.
#[derive(Debug)]
pub enum IoError {
    AddrInUse,
    InvalidInput,
    HostUnreachable,
    UnsupportedAddressFamily,
    AddrNotAvailable,
    ProtocolError,
    NotConnected,
    NoBufferSpace,
    NoMemory,
    Other,
}

/// Converts a C errno value into an IoError type.
pub fn decode_errno(errno: i32) -> IoError {
    // https://github.com/rust-lang/rust/blob/10f6a5c4431e09d355a0ba319a630e02a1e38361/src/libstd/sys/unix/mod.rs#L106
    match errno as libc::c_int {
        ::ffi::XEADDRINUSE => IoError::AddrInUse,
        ::ffi::XEINVAL => IoError::InvalidInput,
        ::ffi::XEHOSTUNREACH => IoError::HostUnreachable,
        ::ffi::XEAFNOSUPPORT => IoError::UnsupportedAddressFamily,
        ::ffi::XENOMEM => IoError::NoMemory,
        ::ffi::XEADDRNOTAVAIL => IoError::AddrNotAvailable,
        ::ffi::XENOBUFS => IoError::NoBufferSpace,
        ::ffi::XEPROTO => IoError::ProtocolError,
        ::ffi::XENOTCONN => IoError::NotConnected,
        _ => IoError::Other,
    }
}
