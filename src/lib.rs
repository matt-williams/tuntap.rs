#![allow(dead_code)]

#[macro_use] extern crate bitflags;
extern crate tokio;
extern crate libc;

mod tuntap;
pub use crate::tuntap::*;
