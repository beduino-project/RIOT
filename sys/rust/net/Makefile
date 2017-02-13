# Auto generate an ffi.rs file from a C header.
RUST_FFI_HEADER = $(RIOTBASE)/sys/include/net/sock/udp.h

# Types from the headers to autogenerate bindings for.
RUST_FFI_TYPES = _sock_tl_ep sock_udp_t sock_udp_ep_t

# Build wrappers to access C macros.
RUST_FFI = 1

include $(RIOTBASE)/sys/rust/Makefile.crate
