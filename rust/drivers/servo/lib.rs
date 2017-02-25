#![no_std]
#![allow(bad_style)]
#![feature(untagged_unions)]

extern crate cpu;

mod ffi;
use ffi::*;

use cpu::libc::c_int;
use cpu::libc::c_uint;

use core::mem;

/// Tuple struct containing a RIOT `servo_t` type.
pub struct Servo(servo_t);

impl Servo {
    /// Creates a new servo tuple struct. This function also calls `servo_init`.
    /// If `servo_init` fails `Option::None` is returned otherwise a tuple struct
    /// instance is returned.
    pub fn new(pwm: pwm_t, ch: c_int, min: c_uint, max: c_uint) -> Option<Self> {
        let mut servo: servo_t = unsafe {
            mem::uninitialized()
        };

        unsafe {
            if servo_init(&mut servo, pwm, ch, min, max) == 0 {
                Option::Some(Servo(servo))
            } else {
                Option::None
            }
        }
    }

    /// Sets the servo motor to a specified position.
    pub fn set(&mut self, pos: c_uint) {
        unsafe {
            servo_set(&mut self.0, pos);
        }
    }
}
