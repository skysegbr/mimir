// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Object type handles are used to represent types such as those created by the SQL command CREATE
//! OR REPLACE TYPE. They are created using the function `Connection::get_object_type()` or
//! implicitly when fetching from a column containing objects by calling the function
//! `Statement::get_query_info()`. Object types are also retrieved when used as attributes in
//! another object by calling the function `ObjectAttribute::get_info()` or as the element type of a
//! collection by calling the function `ObjectType::get_info()`. They are destroyed when the last
//! reference is released by calling the function `ObjectType::release()`.
use error::{ErrorKind, Result};
use object::Object;
use odpi::externs;
use odpi::opaque::{ODPIObject, ODPIObjectType};
use odpi::structs::ODPIObjectTypeInfo;
use std::ptr;

/// Object type handles are used to represent types such as those created by the SQL command CREATE
/// OR REPLACE TYPE.
pub struct ObjectType {
    /// A pointer to the opaque `ODPIObjectType`.
    inner: *mut ODPIObjectType,
}

impl ObjectType {
    /// Get the pointer to the inner ODPI struct.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIObjectType {
        self.inner
    }

    /// Adds a reference to the object type. This is intended for situations where a reference to
    /// the object type needs to be maintained independently of the reference returned when the
    /// object type was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiObjectType_addRef(self.inner),
                 Ok(()),
                 ErrorKind::ObjectType("dpiObjectType_addRef".to_string()))
    }

    /// Creates an object of the specified type and returns a reference to it. This reference should
    ///  be released as soon as it is no longer needed.
    pub fn create(&self) -> Result<Object> {
        let mut object: *mut ODPIObject = ptr::null_mut();

        try_dpi!(externs::dpiObjectType_createObject(self.inner, &mut object),
                 Ok(object.into()),
                 ErrorKind::ObjectType("dpiObjectType_createObject".to_string()))
    }

    /// Returns the list of attributes that belong to the object type.
    /// TODO: Create array when ObjectAttribute is finished.
    pub fn get_attributes(&self, num: u16) -> Result<()> {
        let mut object_attr_arr = ptr::null_mut();

        try_dpi!(externs::dpiObjectType_getAttributes(self.inner, num, &mut object_attr_arr),
                 Ok(()),
                 ErrorKind::ObjectType("dpiObjectType_getAttributes".to_string()))
    }

    /// Returns information about the object type.
    pub fn get_info(&self) -> Result<ODPIObjectTypeInfo> {
        let mut object_type_info: ODPIObjectTypeInfo = Default::default();

        try_dpi!(externs::dpiObjectType_getInfo(self.inner, &mut object_type_info),
                 Ok(object_type_info),
                 ErrorKind::ObjectType("dpiObjectType_getInfo".to_string()))
    }

    /// Releases a reference to the object type. A count of the references to the object type is
    /// maintained and when this count reaches zero, the memory associated with the object type is
    /// freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiObjectType_release(self.inner),
                 Ok(()),
                 ErrorKind::ObjectType("dpiObjectType_release".to_string()))
    }
}

impl From<*mut ODPIObjectType> for ObjectType {
    fn from(oot: *mut ODPIObjectType) -> ObjectType {
        ObjectType { inner: oot }
    }
}

#[cfg(test)]
mod test {
    use connection::Connection;
    use context::Context;
    use error::Result;
    use odpi::flags;
    use std::ffi::CString;
    use test::CREDS;
    use util::ODPIStr;

    fn within_context(ctxt: &Context) -> Result<()> {
        use std::io::{self, Write};
        let mut ccp = ctxt.init_common_create_params()?;
        let enc_cstr = CString::new("UTF-8").expect("badness");
        ccp.set_encoding(enc_cstr.as_ptr());
        ccp.set_nchar_encoding(enc_cstr.as_ptr());

        let conn = Connection::create(&ctxt,
                                      Some(&CREDS[2]),
                                      Some(&CREDS[3]),
                                      Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                                      Some(ccp),
                                      None)?;

        conn.add_ref()?;

        // Query with object
        let object_col = conn.prepare_stmt(Some("select ObjectCol \
                                                 from TestObjects \
                                                 order by IntCol"),
                                           None,
                                           false)?;

        let cols = object_col.execute(flags::EXEC_DEFAULT)?;
        assert_eq!(cols, 1);

        let query_info = object_col.get_query_info(1)?;
        assert!(query_info.object_type().is_some());

        if let Some(object_type) = query_info.object_type() {
            let type_info = object_type.get_info()?;
            let schema = ODPIStr::new(type_info.schema, type_info.schema_length);
            let name = ODPIStr::new(type_info.name, type_info.name_length);
            let schema_str: String = schema.into();
            let name_str: String = name.into();
            writeln!(io::stderr(), "{}:{}", schema_str, name_str)?;
        }

        object_col.close(None)?;
        conn.release()?;
        conn.close(flags::DPI_MODE_CONN_CLOSE_DEFAULT, None)?;

        Ok(())
    }

    fn object_type_res() -> Result<()> {
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
    pub fn object_type() {
        use std::io::{self, Write};

        match object_type_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
