// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Object attribute handles are used to represent the attributes of types such as those created by
//! the SQL command CREATE OR REPLACE TYPE. They are created by calling the function
//! `ODPIObjectType:g:et_attributes()` and are destroyed when the last reference is released by
//! calling the function `ODPIObjectAttr::release()`.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPIObjectAttr;
use odpi::structs::ODPIObjectAttrInfo;

/// Object type handles are used to represent types such as those created by the SQL command CREATE
/// OR REPLACE TYPE.
#[derive(Debug)]
pub struct ObjectAttr {
    /// A pointer to the opaque `ODPIObjectAttr`.
    inner: *mut ODPIObjectAttr,
}

impl ObjectAttr {
    /// Get the pointer to the inner ODPI struct.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIObjectAttr {
        self.inner
    }

    /// Adds a reference to the attribute. This is intended for situations where a reference to the
    /// attribute needs to be maintained independently of the reference returned when the attribute
    /// was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiObjectAttr_addRef(self.inner),
                 Ok(()),
                 ErrorKind::ObjectType("dpiObjectAttr_addRef".to_string()))
    }

    /// Returns information about the attribute.
    pub fn get_info(&self) -> Result<ODPIObjectAttrInfo> {
        let mut object_attr_info: ODPIObjectAttrInfo = Default::default();

        try_dpi!(externs::dpiObjectAttr_getInfo(self.inner, &mut object_attr_info),
                 Ok(object_attr_info),
                 ErrorKind::ObjectType("dpiObjectAttr_getInfo".to_string()))
    }

    /// Releases a reference to the attribute. A count of the references to the attribute is
    /// maintained and when this count reaches zero, the memory associated with the attribute is
    /// freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiObjectAttr_release(self.inner),
                 Ok(()),
                 ErrorKind::ObjectType("dpiObjectAttr_release".to_string()))
    }
}

impl From<*mut ODPIObjectAttr> for ObjectAttr {
    fn from(oot: *mut ODPIObjectAttr) -> ObjectAttr {
        ObjectAttr { inner: oot }
    }
}

#[cfg(test)]
mod test {
    use connection::Connection;
    use context::Context;
    use error::Result;
    // use odpi::enums::ODPIOracleTypeNum::*;
    use odpi::enums::ODPINativeTypeNum::*;
    use odpi::flags;
    use std::ffi::CString;
    use test::CREDS;
    use util::ODPIStr;

    fn within_context(ctxt: &Context) -> Result<()> {
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

            assert_eq!(schema_str, "ODPIC");
            assert_eq!(name_str, "UDT_OBJECT");
            assert_eq!(type_info.is_collection, 0);
            // assert_eq!(type_info.element_oracle_type_num, Max);
            assert_eq!(type_info.element_default_native_type_num, Invalid);
            assert!(type_info.element_object_type.is_null());
            assert_eq!(type_info.num_attributes, 7);

            let _attrs = object_type.get_attributes(7)?;
            // let attr_name = ODPIStr::new(attrs[0].name, attrs[0].name_length);
        }

        object_col.close(None)?;
        conn.release()?;
        conn.close(flags::DPI_MODE_CONN_CLOSE_DEFAULT, None)?;

        Ok(())
    }

    fn object_attr_res() -> Result<()> {
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
    pub fn object_attr() {
        use std::io::{self, Write};

        match object_attr_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
