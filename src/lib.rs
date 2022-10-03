#![cfg_attr(not(feature = "std"), no_std)]
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
)]

/* When depending on std, use the log library, otherwise just stub the macros to nop */
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

#[cfg(feature = "std")]
extern crate core;

#[cfg(not(feature = "std"))]
extern crate core2;

#[cfg(not(feature = "std"))]
#[macro_use]
extern crate alloc;

extern crate crc as crc32;

#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use core2::io;

use core::fmt::{self, Formatter};

#[cfg(feature = "std")]
use std::{vec, vec::Vec};

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    UnexpectedZcrcByte(u8),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO Error {}", e),
            Error::UnexpectedZcrcByte(b) => write!(f, "Unexpected ZCRC byte: {:02X}", b),
        }
    }
}

mod consts;
mod frame;
mod crc;
mod proto;

pub mod recv;
pub mod send;
