#![no_std]
#![allow(bad_style)]
#![feature(untagged_unions)]
#![feature(lang_items)]

extern crate cpu;

pub mod error;
pub mod msg;

pub mod lang_items;
mod ffi;
