#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod models;

#[cfg(feature = "cache")]
pub mod cache;
#[cfg(feature = "config")]
pub mod config;
// #[cfg(crate_type = "cdylib")]
// pub mod c;
// #[cfg(crate_type = "cdylib")]
// pub use crate::c::*;

/// rustc requires a main function to compile a cdylib binary
#[doc(hidden)]
#[allow(dead_code)]
fn main() {}
