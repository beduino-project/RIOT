MODULE = rust/kernel

# Make sure the cpu crate is build before.
DIRS = $(RIOTCPU)/rust/$(CPU)

# Auto generate an ffi.rs file from a C header.
RUST_FFI_HEADER = ffi.h

# Variables form the headers to autogenerate bindings for.
RUST_FFI_VARS = XEADDRINUSE XEINVAL XEHOSTUNREACH XEAFNOSUPPORT \
		XENOMEM XEADDRNOTAVAIL XENOBUFS XEPROTO XENOTCONN

include $(RIOTBASE)/Makefile.base
