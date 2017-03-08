#![feature(lang_items)]
#![feature(const_fn)]
#![no_main]
#![no_std]

extern crate cpu;
extern crate gcoap;
extern crate kernel;

use core::ptr;
use gcoap::flags;

use cpu::libc::c_int;
use cpu::libc::c_char;

extern {
    fn _netif_config(argc: c_int, argv: *mut u8);
}

static mut RESOURCES: [gcoap::resource; 2] = [
    gcoap::resource {
        path: ptr::null(),
        methods: 0,
        handler: Option::None,
    } ; 2
];

static mut LISTENER: gcoap::listener = gcoap::listener {
    resources: ptr::null_mut(),
    resources_len: 0,
    next: ptr::null_mut(),
};

mod handler {
    extern crate gcoap;

    use core::fmt::Write;
    use core::str;

    use gcoap::codes;
    use gcoap::formats;
    use gcoap::resp;
    use gcoap::req;

    pub extern fn hello(p: *mut gcoap::pkt, b: *mut u8, l: usize) -> i32 {
        let mut resp = resp::Response::new(p, b, l, codes::CODE_CONTENT);
        match resp.write("Hello World!".as_ref()) {
            Some(_) => resp.finish(formats::TEXT).unwrap_or(-1) as i32,
            None    => -1
        }
    }

    pub extern fn name(p: *mut gcoap::pkt, b: *mut u8, l: usize) -> i32 {
        let mut req = req::Request::from_pkt(p);
        let mut input: [u8; 32] = [0; 32];

        let len = req.copy_payload(&mut input);
        if len == 0 {
            let mut r = resp::Response::new(p, b, l, codes::CODE_BAD_REQUEST);
            return r.finish(formats::NONE).unwrap_or(-1) as i32;
        };

        let name = match str::from_utf8(input[0..len].as_ref()) {
            Result::Ok(n)  => n,
            Result::Err(_) => return -1,
        };

        let mut resp = resp::Response::new(p, b, l, codes::CODE_CONTENT);
        if write!(resp, "Hello {}! How are you doing?", name).is_err() {
            return -1;
        }

        resp.finish(formats::TEXT).unwrap_or(-1) as i32
    }
}

#[no_mangle]
pub fn main() {
    unsafe {
        _netif_config(0, ptr::null_mut())
    };

    unsafe {
        RESOURCES[0] = gcoap::resource {
            path: b"/hello\0".as_ptr() as *const c_char,
            methods: flags::GET,
            handler: Option::Some(handler::hello),
        };

        RESOURCES[1] = gcoap::resource {
            path: b"/name\0".as_ptr() as *const c_char,
            methods: flags::POST,
            handler: Option::Some(handler::name),
        };

        LISTENER = gcoap::listener {
            resources: RESOURCES.as_mut_ptr(),
            resources_len: RESOURCES.len(),
            next: ptr::null_mut(),
        };

        gcoap::register_listener(&mut LISTENER);
    }
}
