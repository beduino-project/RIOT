#![no_std]

extern crate cpu;

use core::fmt;
use core::fmt::Result;
use core::result;

mod ffi;
use ffi::print;

/// A handle to the global standard output stream of the current process.
///
/// This type is similar to `std::io::Stdout` except that it implements
/// `core::fmt::Write` instead of `std::io::Write`.
pub struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe { print(s.as_ptr(), s.len()); }
        result::Result::Ok(())
    }
}
