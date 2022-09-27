#![no_std]
#![deny(
    //warnings,
    missing_copy_implementations,
    missing_debug_implementations,
    //missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[cfg(feature = "std")]
#[macro_use]
extern crate log;

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! log_enabled {
    ( $( $x:expr )* ) => {
        {
            false
        }
    };
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! error {
    // error!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // error!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ({ });

    // error!("a {} event", "log")
    ($($arg:tt)+) => ({ });
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! debug {
    // debug!(target: "my_target", key1 = 42, key2 = true; "a {} event", "log")
    // debug!(target: "my_target", "a {} event", "log")
    (target: $target:expr, $($arg:tt)+) => ({ });

    // debug!("a {} event", "log")
    ($($arg:tt)+) => ({ });
}

extern crate core2;
extern crate alloc;
extern crate crc as crc32;
#[cfg(feature = "std")]
extern crate hex;

use core2::io;
use core::fmt::{self, Formatter};
use alloc::boxed::Box;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(Box<io::Error>),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(Box::new(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Error!")
    }
}

mod consts;
mod frame;
mod crc;
mod proto;
mod rwlog;

pub mod recv;
pub mod send;
