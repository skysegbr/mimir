// Copyright (c) 2017 mimir developers
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
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;

extern crate chrono;

mod common;
mod connection;
mod context;
mod data;
mod odpi;
mod util;

// Public API
pub mod dequeue;
pub mod enqueue;
#[allow(missing_docs)]
pub mod error;
pub mod lob;
pub mod message;
pub mod object;
pub mod objectattr;
pub mod objecttype;
pub mod pool;
pub mod query;
pub mod rowid;
pub mod statement;
pub mod subscription;
pub mod variable;

// Flattened Public API
pub use connection::Connection;
pub use context::Context;
pub use context::params::AppContext;
pub use data::Data;
pub use odpi::{constants, enums, flags};
pub use odpi::structs::{ODPIBytes, ODPIData, ODPIDataValueUnion, ODPIObjectAttrInfo,
                        ODPIObjectTypeInfo, ODPISubscrMessage};
pub use util::ODPIStr;
