#![allow(bad_style, deprecated, clippy::all)]

use libc::*;
use openssl_sys_10_55::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
