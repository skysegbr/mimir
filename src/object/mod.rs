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
use objectattr::ObjectAttr;
use odpi::{externs, structs};
use odpi::opaque::ODPIObject;

/// This structure represents instances of the types created by the SQL command CREATE OR REPLACE
/// TYPE
#[derive(Clone, Debug)]
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

    /// Returns the value of one of the object's attributes.
    pub fn get_attribute_value(&self,
                               attr: &ObjectAttr,
                               info: &structs::ODPIObjectAttrInfo)
                               -> Result<structs::ODPIData> {
        let mut data_blah: structs::ODPIData = Default::default();

        try_dpi!(externs::dpiObject_getAttributeValue(self.inner,
                                                      attr.inner(),
                                                      info.default_native_type_num,
                                                      &mut data_blah),
                 {
                     Ok(data_blah)
                 },
                 ErrorKind::Object("dpiObject_getAttributeValue".to_string()))
    }

    /// Returns the first index used in a collection.
    pub fn get_first_index(&self) -> Result<(i32, bool)> {
        let mut idx = 0;
        let mut exists = 0;

        try_dpi!(externs::dpiObject_getFirstIndex(self.inner, &mut idx, &mut exists),
                 Ok((idx, exists == 1)),
                 ErrorKind::Object("dpiObject_getFirstIndex".to_string()))
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
