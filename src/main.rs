//! `hello_world` prints "Hello world" to the serial port on a Sipeed Maixduino M1 board.

#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
#![no_std]
#![no_main]
// Uncomment before ship to discover possibly redundant crates, debug remnants, missing licenses...
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod app;
mod consts;
mod error;
mod panic;

use core::sync::atomic::{self, Ordering};
use riscv_rt::entry;
use {consts::*, error::Error};

/// `Result<T>` alias which defaults to `crate::Error` as the error type.
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Program execution begins here.
#[entry]
fn main() -> ! {
    let res = app::run();

    loop {
        if let Err(ref err) = res {
            display_err(err)
        }
        delay();
    }
}

fn delay() {
    (0..LOOP_DELAY).for_each(|_| atomic::compiler_fence(Ordering::SeqCst))
}

fn display_err(_err: &Error) {
    // Communicate error state (e.g. flash LEDs in a pattern)
}
