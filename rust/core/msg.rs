use cpu::libc::c_int;

use ::ffi::*;
pub use ::ffi::msg_t;
pub use ::ffi::msg_t__bindgen_ty_1 as content;

impl msg_t {
    /// Create a new message of the given type with the given content.
    pub fn new(t: u16, c: content) -> Self {
        msg_t { sender_pid: 0, type_: t, content: c }
    }
}

/// Enum type used to differentiate between different errors that can
/// occur when trying to send an IPC message to another thread.
pub enum SendError {
    ReceiverNotReady,
    InvalidPID,
}

/// Initializes a message queue for the current thread. This is needed
/// if you want to do asynchronous IPC.
pub fn init_queue(m: &mut [msg_t]) {
    let len = m.len() as c_int;
    unsafe {
        msg_init_queue(m.as_mut_ptr(), len)
    }
}

/// Sends a message to another thread. If called from an interrupt, this
/// function will never block.
pub fn send(m: &mut msg_t, p: kernel_pid_t) -> Result<(), SendError> {
    let r = unsafe {
        msg_send(m, p)
    };

    match r {
        0  => Result::Err(SendError::ReceiverNotReady),
        -1 => Result::Err(SendError::InvalidPID),
        _  => Result::Ok(()),
    }
}

/// Sends a message to the own thread. This will only work if the
/// message queue has been initialized before hand.
pub fn send_to_self(m: &mut msg_t) -> Option<()> {
    let r = unsafe {
        msg_send_to_self(m)
    };

    if r == 1 {
        Option::Some(())
    } else {
        Option::None
    }
}

/// Receives an IPC message and stores it in the given mutable `msg_t`
/// reference. It will block until a message is received.
pub fn receive(m: &mut msg_t) {
    unsafe {
        msg_receive(m)
    };
}

/// Sends a message to a target and then blocks until the target sent a
/// reply. The reply is stored in the mutable `msg_t` reference passed
/// as the `re` parameter.
pub fn send_receive(m: &mut msg_t, re: &mut msg_t, p: kernel_pid_t) {
    unsafe {
        msg_send_receive(m, re, p)
    };
}

/// Replies to a message. The message which you want to reply to must
/// have been sent by the sender using the `msg_send_receive` function.
pub fn reply(m: &mut msg_t, re: &mut msg_t) -> Option<()> {
    let r = unsafe {
        msg_reply(m, re)
    };

    if r == -1 {
        Option::None
    } else {
        Option::Some(())
    }
}

/// Returns the amount of messages available in the current threads
/// queue or `Option::None` if the threads queue wasn't initialized
/// beforehand using `msg_init_queue`.
pub fn available() -> Option<c_int> {
    let r = unsafe {
        msg_avail()
    };

    if r == -1 {
        Option::None
    } else {
        Option::Some(r)
    }
}

/// Prints the message queue of the current thread.
pub fn print_queue() {
    unsafe {
        msg_queue_print();
    }
}
