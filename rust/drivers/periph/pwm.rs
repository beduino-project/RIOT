#![allow(bad_style)]

extern crate cpu;
use cpu::libc::c_int;

pub use ::ffi::pwm_t;
pub use ::ffi::pwm_mode_t;

use ::ffi::pwm_init;
use ::ffi::pwm_channels;
use ::ffi::pwm_set;
use ::ffi::pwm_poweron;
use ::ffi::pwm_poweroff;

extern {
    fn pwm_dev(x: c_int) -> pwm_t;
}

/// Struct representing a RIOT pwm devices. This is a wrapper for the
/// `pwm_t` RIOT type.
pub struct Pwm(pwm_t);

/// Wrapper for the `PWM_DEV` macro.
pub fn dev(x: c_int) -> pwm_t {
    unsafe {
        pwm_dev(x)
    }
}

impl Pwm {
    /// Initializes the given PWM devices. If the desired resolution and
    /// frequency isn't possible on the given device the PWM driver will
    /// decrease the frequency. The return value is thus a tuple of the
    /// actual frequency and an instanz of the Pwm type.
    ///
    /// If an error occurs `Option::None` is returned.
    pub fn new(dev: pwm_t, mode: pwm_mode_t, freq: u32, res: u16)
            -> Option<(Self, u32)> {
        let ret = unsafe {
            pwm_init(dev, mode, freq, res)
        };

        if ret == 0 {
            Option::None
        } else {
            Option::Some((Pwm(dev), ret))
        }
    }

    /// Returns the number of channels available for this PWM device.
    pub fn get_channels(self) -> u8 {
        unsafe {
            pwm_channels(self.0)
        }
    }

    /// Sets the duty-cycle for a given channel of this PWM device.
    pub fn set(self, ch: u8, val: u16) {
        unsafe {
            pwm_set(self.0, ch, val)
        }
    }

    /// Resume PWM generation on on this device.
    pub fn poweron(self) {
        unsafe {
            pwm_poweron(self.0)
        }
    }

    /// Stop PWM generation on this PWM device.
    pub fn poweroff(self) {
        unsafe {
            pwm_poweroff(self.0)
        }
    }
}
