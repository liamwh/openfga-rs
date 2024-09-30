#![warn(missing_debug_implementations, rust_2018_idioms, unreachable_pub)]
#![forbid(unsafe_code)]

pub use prost_types;
pub use tonic;
pub mod authentication;

include!(concat!(env!("OUT_DIR"), "/openfga.v1.rs"));
