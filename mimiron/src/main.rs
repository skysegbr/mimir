// Copyright (c) 2017 mimiron developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `mimiron` 0.1.0
#![deny(missing_docs)]
#![recursion_limit="128"]
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
mod macros;

extern crate clap;
extern crate rusoto_core;
extern crate rusoto_rds;
extern crate term;

mod error;
mod event;
mod run;

use std::io::{self, Write};
use std::process;

lazy_static! {
    static ref SOURCE_TYPES: Vec<&'static str> = {
        let mut st = Vec::new();
        st.extend(&["db-instance",
                    "db-parameter-group",
                    "db-security-group",
                    "db-snapshot",
                    "db-cluster",
                    "db-cluster-snapshot"]);
        st
    };
}

/// CLI Entry Point
fn main() {
    match run::run() {
        Ok(i) => process::exit(i),
        Err(e) => {
            writeln!(io::stderr(), "{}", e).expect("Unable to write to stderr!");
            process::exit(1)
        }
    }
}
