use cpu::libc::c_uint;

use core::fmt;
use core::slice;

use ::ffi::gcoap_finish;
use ::ffi::gcoap_resp_init;

/// This datatype should be used within a CoAP handler to setup and send
/// a response. It contains some addational abstractions for the
/// `gcoap_resp_init()` and `gcoap_finish()` functions.
pub struct Response {
    pkt: *mut ::pkt,
    ptr: *const u8,
}

impl Response {
    /// Creates a new CoAP respones from a given PDU, a buffer
    /// containing the PDU and a response code (see `gcoap::codes` for
    /// available response codes).
    ///
    /// The parameters required by this function are equal to the once
    /// passed to a response handler.
    pub fn new(p: *mut ::pkt, b: *mut u8, l: usize, c: c_uint) -> Self {
        unsafe {
            gcoap_resp_init(p, b, l, c)
        };

        unsafe {
            Response { pkt: p, ptr: (*p).payload }
        }
    }

    /// Writes the given data to the payload body. The amount of bytes
    /// actually written is returned. Incase the length of the data
    /// exceeds the max length of the payload `Option::None` is
    /// returned.
    pub fn write(&mut self, data: &[u8]) -> Option<usize> {
        let len = unsafe {
            (*self.pkt).payload_len as usize -
                ((*self.pkt).payload as usize - self.ptr as usize)
        };

        if data.len() > len {
            return Option::None;
        }

        let mut payload = unsafe {
            slice::from_raw_parts_mut((*self.pkt).payload, data.len())
        };

        payload.copy_from_slice(data);
        unsafe {
            (*self.pkt).payload = (*self.pkt).payload
                .offset(data.len() as isize);
        };

        Option::Some(data.len())
    }

    /// Send the response with the given format to the client. Valid
    /// formats are defined in `gcoap::formats`. If an error occurs
    /// `Option::None` is returned.
    pub fn finish(&mut self, f: c_uint) -> Option<isize> {
        let len = unsafe {
            (*self.pkt).payload as usize - self.ptr as usize
        };

        let ret = unsafe {
            (*self.pkt).payload = self.ptr as *mut u8;
            gcoap_finish(self.pkt, len, f)
        };

        if ret == 0 {
            Option::None
        } else {
            Option::Some(ret)
        }
    }
}

impl fmt::Write for Response {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.write(s.as_bytes()) {
            Some(_) => Result::Ok(()),
            None    => Result::Err(fmt::Error)
        }
    }
}
