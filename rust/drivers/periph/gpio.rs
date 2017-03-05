use cpu::libc::c_int;

pub use ::ffi::gpio_t;
pub use ::ffi::gpio_mode_t;

use ::ffi::gpio_init;
use ::ffi::gpio_read;
use ::ffi::gpio_write;
use ::ffi::gpio_toggle;

/// Struct representing a GPIO pin.
pub struct Pin(gpio_t);

extern {
    fn gpio_pin(x: c_int, y: c_int) -> gpio_t;
}

impl Pin {
    /// Creates a new GPIO Pin with the provided number `n`.
    pub fn new(n: u32) -> Pin {
        Pin(n)
    }

    /// Creates a new GPIO Pin from a port-pin tuple. `x` is
    /// the port which should be just and `y` is the pin which
    /// should be used.
    pub fn from_tuple(x: i32, y: i32) -> Pin {
        Pin(unsafe { gpio_pin(x, y) })
    }

    /// Initialize the pin as a general purpose input or output.
    pub fn init(&self, mode: gpio_mode_t) -> Option<()> {
        let r = unsafe {
            gpio_init(self.0, mode)
        };

        if r == -1 {
            Option::None
        } else {
            Option::Some(())
        }
    }

    /// Returns the current value of the pin.
    ///
    /// True is returned if the pin is high, otherwise it is low.
    pub fn get_value(&self) -> bool {
        unsafe {
            gpio_read(self.0) != 0
        }
    }

    /// Sets the given `pin` to the given value `val`.
    ///
    /// If `val` is true than the pin is set to high, otherwise
    /// (if it is false) it is set to low.
    pub fn set_value(&self, val: bool) {
        unsafe {
            let val = if val { 1 } else { 0 };
            gpio_write(self.0, val)
        }
    }

    // Toggle the value of the pin.
    pub fn toggle_value(&self) {
        unsafe {
            gpio_toggle(self.0)
        }
    }
}
