#![no_std]

extern crate cpu;
use cpu::libc::{size_t, c_char};

use core::fmt;
use core::fmt::Result;
use core::result;

extern {
    fn print(ptr: *const c_char, len: size_t) -> ();
}

// A handle to the global standard output stream of the current process.
//
// This type is similar to `std::io::Stdout` except that it implements
// `core::fmt::Write` instead of `std::io::Write`.
pub struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        unsafe { print(s.as_ptr(), s.len()); }
        result::Result::Ok(())
    }
}
