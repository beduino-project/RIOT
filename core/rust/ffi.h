#include <errno.h>

/**
 * This file is needed because values in errno.h are usually defined as
 * C macros. Bindgen is able to 'parse' basic C macros like `#define foo
 * 42` using a regex but this is a) unrealiable and b) bindgen uses the
 * c_uint type to represent this value but we actually need a c_int.
 * Thus we decided to wrap these values maunually.
 *
 * TODO: Currently the values have to be added here and in the Makefile.
 * Maybe we should autogenerate the ffi.h from the Makefile?
 */

static const int XEADDRINUSE = EADDRINUSE;
static const int XEINVAL = EINVAL;
static const int XEHOSTUNREACH = EHOSTUNREACH;
static const int XEAFNOSUPPORT = EAFNOSUPPORT;
static const int XENOMEM = ENOMEM;
static const int XEADDRNOTAVAIL = EADDRNOTAVAIL;
static const int XENOBUFS = ENOBUFS;
static const int XEPROTO = EPROTO;
static const int XENOTCONN = ENOTCONN;
