#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")]
extern crate test;

#[macro_use]
extern crate lazy_static;
pub use lazy_static::lazy_static;

extern crate log;
pub use log::{debug, error, info, trace, warn};

pub mod block;
pub mod chunk;
pub mod events;
pub mod utils;
pub mod world;
