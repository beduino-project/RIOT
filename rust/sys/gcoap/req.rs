use core::slice;
use core::cmp::min;

/// This datatype should be used within a CoAP hander to parse a client
/// request. It provides some abstractions around `coap_pkt_t` for
/// accessing the payload.
pub struct Request(*mut ::pkt);

impl Request {
    /// Create a new request from a `coap_pkt_t` value.
    pub fn from_pkt(p: *mut ::pkt) -> Self {
        Request(p)
    }

    /// Returns a reference to the payload. This unsafe because the
    /// content of the buffer pointed to by the reference can change
    /// if another method (e.g. Response::Write) changes the payload
    /// field of the coap_pkt_t struct.
    pub unsafe fn get_payload(&mut self) -> &[u8] {
        slice::from_raw_parts((*self.0).payload,
            (*self.0).payload_len as usize)
    }

    /// Copies the request payload to the given buffer. The entire
    /// payload may not be copied to the buffer if the buffer is to
    /// small to hold it. To detect this the amount of bytes copied to
    /// the buffer are returned by this function.
    pub fn copy_payload(&mut self, buf: &mut [u8]) -> usize {
        let payload = unsafe {
            self.get_payload()
        };

        let len = min(payload.len(), buf.len());
        buf[..len].copy_from_slice(payload[..len].as_ref());
        len
    }
}
