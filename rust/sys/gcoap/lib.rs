#![no_std]
#![allow(bad_style)]

extern crate cpu;

pub use ::ffi::coap_pkt_t as pkt;
pub use ::ffi::gcoap_listener_t as listener;
pub use ::ffi::coap_resource_t as resource;

mod ffi;

pub fn register_listener(l: *mut listener) {
    unsafe { ffi::gcoap_register_listener(l) }
}

pub mod req;
pub mod resp;

pub mod codes;
pub mod flags;
pub mod formats;
