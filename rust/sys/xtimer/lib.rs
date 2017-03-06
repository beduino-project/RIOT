#![no_std]

extern crate cpu;

extern {
    fn xsleep(s: u32);
    fn xusleep(ms: u32);
    fn xnanosleep(ns: u32);
}

/// A duration type to represent a span of time, typically
/// used for system timeouts.
pub enum Duration {
    // A duration in seconds.
    Seconds(u32),

    // A duration in milliseconds.
    Milliseconds(u32),

    // A duration in nanoseconds.
    Nanoseconds(u32),
}

use Duration::*;

/// Puts the current thread to sleep for the specified duration.
pub fn sleep(dur: Duration) {
    unsafe {
        match dur {
            Seconds(n)      => xsleep(n),
            Milliseconds(n) => xusleep(n),
            Nanoseconds(n)  => xnanosleep(n),
        }
    }
}
