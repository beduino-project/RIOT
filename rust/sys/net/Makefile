# Auto generate an ffi.rs file from a C header.
RUST_FFI_HEADER = $(RIOTBASE)/sys/include/net/sock/udp.h

# Types from the headers to autogenerate bindings for.
RUST_FFI_TYPES = _sock_tl_ep sock_udp_t sock_udp_ep_t

# Functions to autogenerate bindings for.
RUST_FFI_FUNCS = sock_udp_create sock_udp_send sock_udp_recv \
		 sock_udp_get_local sock_udp_close

# Build wrappers to access C macros.
RUST_FFI = 1

include $(RIOTRUST)/sys/Makefile.crate
