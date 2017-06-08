// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Rust bindings over the Oracle Database Programming Interface for Drivers and Applications.
#![deny(missing_docs)]
#![feature(untagged_unions)]
#![recursion_limit="128"]
#![cfg_attr(feature = "cargo-clippy", allow(unseparated_literal_suffix))]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate getset;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;

extern crate chrono;
#[cfg(test)]
extern crate rand;

// Public API
pub mod common;
pub mod connection;
pub mod context;
pub mod data;
pub mod dequeue;
pub mod enqueue;
#[allow(missing_docs)]
pub mod error;
pub mod lob;
pub mod message;
pub mod object;
pub mod objecttype;
pub mod pool;
pub mod query;
pub mod rowid;
pub mod statement;
pub mod subscription;
pub mod variable;

mod odpi;
mod util;

pub use odpi::{constants, enums, flags};
pub use odpi::structs::ODPIDataValueUnion as DataUnion;

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[cfg(test)]
    lazy_static! {
        pub static ref CREDS: Vec<String> = {
            let file = File::open(".creds/oic-test")
                .expect("bad creds");
            let mut buf_reader = BufReader::new(file);
            let mut creds = String::new();
            let _ = buf_reader.read_line(&mut creds).expect("bad creds");
            creds.split(":").map(|x| x.trim_right().to_string()).collect()
        };
    }
}
