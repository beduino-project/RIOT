# Introduction

This example shows how to write a RIOT application using Rust.

# Status

As Rust's libstd is too fat for our little MCUs, it is currently only
possible to write Rust applications using `#![no_std]`. However, crates
to map RIOT's API to rust are available in `sys/rust`.

# Prerequisites

Since building binary packages with `#![no_std]` is currently not
supported by the rust stable channel you need a nightly rust toolchain.

After installing the nightly rust toolchain you should be good to go.
The nightly version is needed because currently it is the only version
supporting builds without the standard library for bin crates.

# Trying the example

As always,

    # make BOARD=native all term

is all you need.
