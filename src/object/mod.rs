// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! This structure represents instances of the types created by the SQL command CREATE OR REPLACE
//! TYPE and is available by handle to a calling application or driver. An object is created by
//! calling the function `ObjectType::createObject()` or by calling the function `Object::copy()`.
//! They are also created implicitly by creating a variable of the type DPI_ORACLE_TYPE_OBJECT.
//! Objects are destroyed when the last reference is released by calling the function
//! `Object::release()`. All of the attributes of the structure `ODPIBaseType` are included in this
//! structure in addition to the ones specific to this structure described below.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPIObject;

/// This structure represents instances of the types created by the SQL command CREATE OR REPLACE
/// TYPE
#[derive(Clone)]
pub struct Object {
    /// The ODPI-C Object pointer.
    pub inner: *mut ODPIObject,
}

impl Object {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIObject {
        self.inner
    }

    /// Adds a reference to the object. This is intended for situations where a reference to the
    /// object needs to be maintained independently of the reference returned when the object was
    /// created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiObject_addRef(self.inner),
                 Ok(()),
                 ErrorKind::Object("dpiObject_addRef".to_string()))
    }

    /// Releases a reference to the object. A count of the references to the object is maintained
    /// and when this count reaches zero, the memory associated with the object is freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiObject_release(self.inner),
                 Ok(()),
                 ErrorKind::Object("dpiObject_release".to_string()))
    }
}

impl From<*mut ODPIObject> for Object {
    fn from(inner: *mut ODPIObject) -> Object {
        Object { inner: inner }
    }
}

#[cfg(test)]
mod test {
    // use chrono::{Datelike, UTC, Timelike};
    use connection::Connection;
    use context::Context;
    use error::Result;
    use odpi::flags;
    // use odpi::enums::ODPIMessageDeliveryMode::*;
    // use odpi::enums::ODPIMessageState::*;
    use std::ffi::CString;
    use test::CREDS;

    fn within_context(ctxt: &Context) -> Result<()> {
        let mut ccp = ctxt.init_common_create_params()?;
        let enc_cstr = CString::new("UTF-8").expect("badness");
        ccp.set_encoding(enc_cstr.as_ptr());
        ccp.set_nchar_encoding(enc_cstr.as_ptr());
        ccp.set_create_mode(flags::DPI_MODE_CREATE_EVENTS);

        let conn = Connection::create(ctxt,
                                      Some(&CREDS[0]),
                                      Some(&CREDS[1]),
                                      Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                                      Some(ccp),
                                      None)?;
        conn.close(flags::DPI_MODE_CONN_CLOSE_DEFAULT, None)?;

        Ok(())
    }

    fn object_res() -> Result<()> {
        use std::io::{self, Write};

        let ctxt = Context::create()?;
        match within_context(&ctxt) {
            Ok(_) => Ok(()),
            Err(e) => {
                writeln!(io::stderr(), "{}", ctxt.get_error())?;
                Err(e)
            }
        }
    }

    #[test]
    fn object() {
        use std::io::{self, Write};

        match object_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
