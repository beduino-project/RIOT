extern crate basecpu;

// See https://github.com/RIOT-OS/RIOT/wiki/Family:-native#data-type-sizes

pub use self::basecpu::libc::*;

pub type c_char = u8;
pub type c_long = i32;
pub type c_ulong = u32;
