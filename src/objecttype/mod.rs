// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

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
use odpi::opaque::{ODPIObject, ODPIObjectAttr, ODPIObjectType};
use odpi::structs::ODPIObjectTypeInfo;
use std::ptr;

/// Object type handles are used to represent types such as those created by the SQL command CREATE
/// OR REPLACE TYPE.
#[derive(Debug)]
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
    pub fn get_attributes(&self, length: u16) -> Result<Vec<*mut ODPIObjectAttr>> {
        let mut buffer: Vec<*mut ODPIObjectAttr> = Vec::with_capacity(length as usize);
        let buf_ptr = buffer.as_mut_ptr();

        try_dpi!(externs::dpiObjectType_getAttributes(self.inner, length, buf_ptr),
                 {
                     unsafe { buffer.set_len(length as usize) };
                     Ok(buffer)
                 },
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
    use chrono::{TimeZone, UTC};
    use connection::Connection;
    use context::Context;
    use data::Data;
    use error::Result;
    use object::Object;
    use objectattr::ObjectAttr;
    use objecttype::ObjectType;
    use odpi::enums;
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
            let attrs = object_type.get_attributes(7)?;
            let mut obj_attrs = Vec::new();
            let mut attr_infos = Vec::new();

            for (idx, obj_attr) in attrs.iter().enumerate() {
                let attr: ObjectAttr = (*obj_attr).into();
                let attr_info = attr.get_info()?;
                let name_s = ODPIStr::new(attr_info.name, attr_info.name_length);
                let name: String = name_s.into();
                match idx {
                    0 => assert_eq!(name, "NUMBERVALUE"),
                    1 => assert_eq!(name, "STRINGVALUE"),
                    2 => assert_eq!(name, "FIXEDCHARVALUE"),
                    3 => assert_eq!(name, "DATEVALUE"),
                    4 => assert_eq!(name, "TIMESTAMPVALUE"),
                    5 => assert_eq!(name, "SUBOBJECTVALUE"),
                    6 => assert_eq!(name, "SUBOBJECTARRAY"),
                    _ => assert!(false),
                }
                obj_attrs.push(attr);
                attr_infos.push(attr_info);
            }

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

            object_col.fetch()?;

            // Create an object of this type.
            let created_obj = object_type.create()?;
            let _created: Object = created_obj.into();
            // let (first_idx, exists) = created.get_first_index()?;
            // assert_eq!(first_idx, 0);
            // assert_eq!(exists, 1);

            // Get the object value out of the query.
            let (object_col_type, object_col_ptr) = object_col.get_query_value(1)?;
            assert_eq!(object_col_type, Object);
            let data: Data = object_col_ptr.into();
            let obj: Object = data.as_object().into();

            for (idx, (obj_attr, attr_info)) in
                obj_attrs.iter().zip(attr_infos.iter()).enumerate() {
                let attr_data = obj.get_attribute_value(obj_attr, attr_info)?;
                match attr_info.default_native_type_num {
                    Bytes => {
                        let data_bytes = unsafe { attr_data.value.as_bytes };
                        let o_str = ODPIStr::new(data_bytes.ptr, data_bytes.length);
                        let data_str: String = o_str.into();
                        if idx == 1 {
                            assert_eq!(data_str, "First row");
                        } else if idx == 2 {
                            assert_eq!(data_str, "First     ");
                        } else {
                            assert!(false);
                        }
                    }
                    Double => {
                        if idx == 0 {
                            assert_eq!(unsafe { attr_data.value.as_double }, 1.0);
                        } else {
                            assert!(false);
                        }
                    }
                    Timestamp => {
                        let odpi_ts = unsafe { attr_data.value.as_timestamp };
                        let y = odpi_ts.year as i32;
                        let m = odpi_ts.month as u32;
                        let d = odpi_ts.day as u32;
                        let h = odpi_ts.hour as u32;
                        let mi = odpi_ts.minute as u32;
                        let s = odpi_ts.second as u32;
                        let ts = UTC.ymd(y, m, d).and_hms_nano(h, mi, s, odpi_ts.fsecond);

                        if idx == 3 {
                            let expected = UTC.ymd(2007, 3, 6).and_hms_nano(0, 0, 0, 0);
                            assert_eq!(ts, expected);
                        } else if idx == 4 {
                            let expected = UTC.ymd(2008, 9, 12).and_hms_nano(16, 40, 0, 0);
                            assert_eq!(ts, expected);
                        } else {
                            assert!(false);
                        }
                    }
                    Object => {
                        let nested_obj_type_ptr = attr_info.object_type;

                        if nested_obj_type_ptr.is_null() {
                            assert!(false);
                        } else {
                            let nested_obj_type: ObjectType = nested_obj_type_ptr.into();
                            let type_info = nested_obj_type.get_info()?;
                            let schema = ODPIStr::new(type_info.schema, type_info.schema_length);
                            let name = ODPIStr::new(type_info.name, type_info.name_length);
                            let schema_str: String = schema.into();
                            let name_str: String = name.into();

                            let odpi_obj_ptr = unsafe { attr_data.value.as_object };
                            let odpi_obj: Object = odpi_obj_ptr.into();

                            assert_eq!(schema_str, "ODPIC");
                            if idx == 5 {
                                assert_eq!(name_str, "UDT_SUBOBJECT");
                                assert_eq!(type_info.is_collection, 0);
                                assert_eq!(type_info.num_attributes, 2);
                            } else if idx == 6 {
                                assert_eq!(name_str, "UDT_OBJECTARRAY");
                                assert_eq!(type_info.is_collection, 1);
                                assert_eq!(type_info.element_oracle_type_num,
                                           enums::ODPIOracleTypeNum::Object);
                                assert_eq!(type_info.element_default_native_type_num, Object);
                                assert!(!type_info.element_object_type.is_null());
                                let arr_obj_type: ObjectType = type_info.element_object_type.into();
                                let type_info = arr_obj_type.get_info()?;
                                let schema = ODPIStr::new(type_info.schema,
                                                          type_info.schema_length);
                                let name = ODPIStr::new(type_info.name, type_info.name_length);
                                let schema_str: String = schema.into();
                                let name_str: String = name.into();

                                assert_eq!(schema_str, "ODPIC");
                                assert_eq!(name_str, "UDT_SUBOBJECT");
                                assert_eq!(type_info.is_collection, 0);
                                assert_eq!(type_info.num_attributes, 2);

                                let (first_index, _exists) = odpi_obj.get_first_index()?;
                                assert_eq!(first_index, 0);
                            }
                        }
                    }
                    _ => {
                        assert!(false);
                    }
                }
            }

            for obj_attr in obj_attrs {
                obj_attr.release()?;
            }
        }

        object_col.close(None)?;
        conn.close(flags::DPI_MODE_CONN_CLOSE_DEFAULT, None)?;
        object_col.release()?;
        conn.release()?;

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
