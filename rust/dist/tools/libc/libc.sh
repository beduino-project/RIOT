#!/bin/sh

# This script outputs code for the C preprocessor that outputs rust
# code that is a replacement for std::os::raw.

# We can't just copy std::os::raw because it is apparently only targeted
# at platforms with at least 32 bit. (Probably because std will never
# fit onto anything using a 16 bit CPU because of limited flash sizes.)

# c_void
#
# The definition of c_void was copied from std::os::raw.
# To pass the rust attribute syntax (beginning with a hash sign)
# through the CPP, we need to jump some hoops.
cat <<EOF
#define hash #
#define id(x) x
#define hashed(x) id(hash)x

hashed([repr(u8)])
pub enum c_void {
	hashed([doc(hidden)]) __variant1,
	hashed([doc(hidden)]) __variant2,
}
EOF

# c_char and c_schar/c_uchar
#
# Those are assumed to always be eight bit in size. c_char might be
# unsigned on some implementations.
cat <<EOF
pub type c_schar = i8;
pub type c_uchar = u8;

#if __CHAR_IS_UNSIGNED__
pub type c_char = u8;
#else
pub type c_char = i8;
#endif
EOF

# all the other stuff
#
# For all types with SIZEOF-macros, we just emit a lengthy curse
# and let the CPP figure things out.

curse() {
	cat <<-EOF

		#if $1 == 1
		pub type $2 = ${3}8;
		#else
		#if $1 == 2
		pub type $2 = ${3}16;
		#else
		#if $1 == 4
		pub type $2 = ${3}32;
		#else
		#if $1 == 8
		pub type $2 = ${3}64;
		#else
		#if $1 == 16
		pub type $2 = ${3}128;
		#else
		#error "Can't handle $1"
		#endif
		#endif
		#endif
		#endif
		#endif
	EOF
}

curse __SIZEOF_SHORT__     c_short     i
curse __SIZEOF_INT__       c_int       i
curse __SIZEOF_LONG__      c_long      i
curse __SIZEOF_LONG_LONG__ c_longlong  i

curse __SIZEOF_SHORT__     c_ushort    u
curse __SIZEOF_INT__       c_uint      u
curse __SIZEOF_LONG__      c_ulong     u
curse __SIZEOF_LONG_LONG__ c_ulonglong u

curse __SIZEOF_FLOAT__     c_float     f
curse __SIZEOF_DOUBLE__    c_double    f

