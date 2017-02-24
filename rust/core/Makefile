MODULE = rust/kernel

# Make sure the cpu crate is build before.
DIRS = $(RIOTRUST)/cpu/$(CPU)

# Auto generate an ffi.rs file from a C header.
RUST_FFI_HEADER = ffi.h

# Variables form the headers to autogenerate bindings for.
RUST_FFI_VARS = XEADDRINUSE XEINVAL XEHOSTUNREACH XEAFNOSUPPORT \
		XENOMEM XEADDRNOTAVAIL XENOBUFS XEPROTO XENOTCONN

# Functions to autogenerate bindings for.
RUST_FFI_FUNCS = msg_init_queue msg_send msg_send_to_self \
		 msg_send_receive msg_receive msg_reply \
		 msg_avail msg_queue_print

# The kernel_pid_t variable is needed by the IPC wrapper.
RUST_FFI_TYPES = kernel_pid_t

include $(RIOTBASE)/Makefile.base
