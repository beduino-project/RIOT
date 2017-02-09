# Introduction

This folder contains RIOT example applications written in Rust.

# Status

As Rust's libstd is too fat for our little MCUs, it is currently only
possible to write Rust applications using `#![no_std]`. However, various
crates to map RIOTs API to Rust are available.

# Prerequisites

Since building binary packages with `#![no_std]` is currently not
supported by the rust stable channel you need a nightly rust toolchain.

After installing the nightly rust toolchain you should be good to go.
The nightly version is needed because it is currently the only version
supporting builds without the standard library for bin crates.

# Upgrading

As explained above we can't use libstd and instead rely on the smaller
libcore. The libcore library is included as a RIOT pkg, it is very
closely tied to the rustc version you are using. Therefore, if you
upgrade (or downgrade) your nightly rustc you need to rebuild libcore.
Before doing so you need to run `make distclean` to force RIOT to
checkout the appropriate libcore version and run `make all` as usual
afterwards.

# Trying the examples

As always,

    # make BOARD=native all term

is all you need.
