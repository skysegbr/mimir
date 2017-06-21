// Copyright (c) 2017 mimir developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Variable handles are used to represent memory areas used for transferring data to and from the
//! database. They are created by calling the function `Connection::newVar()`. They are destroyed
//! when the last reference to the variable is released by calling the function `release()`. They
//! are bound to statements by calling the function `Statement::bindByName()` or the function
//! `Statement::bindByPos()`. They can also be used for fetching data from the database by calling
//! the function `Statement::define()`.
use error::{ErrorKind, Result};
use lob::Lob;
use object::Object;
use odpi::externs;
use odpi::opaque::ODPIVar;
use odpi::structs::ODPIData;
use rowid::Rowid;
use statement::Statement;
use std::{ptr, slice};
use util::ODPIStr;

/// This structure represents memory areas used for transferring data to and from the database and
/// is available by handle to a calling application or driver.
pub struct Var {
    /// The ODPI-C var
    inner: *mut ODPIVar,
}

impl Var {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIVar {
        self.inner
    }

    /// Adds a reference to the variable. This is intended for situations where a reference to the
    /// variable needs to be maintained independently of the reference returned when the variable
    /// was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiVar_addRef(self.inner),
                 Ok(()),
                 ErrorKind::Var("dpiVar_addRef".to_string()))
    }

    /// Copies the data from one variable to another variable.
    ///
    /// * `src_pos` - the array position from which the data is to be copied. The first position is
    /// 0. If the array position specified exceeds the number of elements allocated in the source
    /// variable, an error is returned.
    /// * `dst` - the variable into which data is to be copied.
    /// * `dst_pos` - the array position into which the data is to be copied. The first position is
    /// 0. If the array position specified exceeds the number of elements allocated in the variable,
    /// an error is returned.
    pub fn copy_data(&self, src_pos: u32, dst: &mut Var, dst_pos: u32) -> Result<()> {
        try_dpi!(externs::dpiVar_copyData(dst.inner(), dst_pos, self.inner, src_pos),
                 Ok(()),
                 ErrorKind::Var("dpiVar_copyData".to_string()))
    }

    /// Returns a pointer to an array of `ODPIData` structures used for transferring data to and
    /// from the database. These structures are allocated by the variable itself and are made
    /// available when the variable is first created using the function `Connection::new_var()`. If
    /// a DML returning statement is executed, however, the number of allocated elements can change
    /// in addition to the memory location.
    pub fn get_data(&self) -> Result<&mut [ODPIData]> {
        let mut num_elements = 0;
        let mut data_arr_ptr = ptr::null_mut();

        try_dpi!(externs::dpiVar_getData(self.inner, &mut num_elements, &mut data_arr_ptr),
                 {
                     if data_arr_ptr.is_null() || num_elements == 0 {
                         Ok(&mut [])
                     } else {
                         Ok(unsafe {
                                slice::from_raw_parts_mut(data_arr_ptr, num_elements as usize)
                            })
                     }
                 },
                 ErrorKind::Var("dpiVar_getData".to_string()))
    }

    /// Returns the number of elements in a PL/SQL index-by table if the variable was created as an
    /// array by the function `Connection::newVar()`. If the variable is one of the output bind
    /// variables of a DML returning statement, however, the value returned will correspond to the
    /// number of rows returned by the DML returning statement. In all other cases, the value
    /// returned will be the number of elements the variable was created with.
    pub fn get_num_elements_in_array(&self) -> Result<u32> {
        let mut num_elements = 0;
        try_dpi!(externs::dpiVar_getNumElementsInArray(self.inner, &mut num_elements),
                 Ok(num_elements),
                 ErrorKind::Var("dpiVar_getNumElementsInArray".to_string()))
    }

    /// Returns the size of the buffer used for one element of the array used for fetching/binding
    /// Oracle data.
    pub fn get_size_in_bytes(&self) -> Result<u32> {
        let mut size = 0;
        try_dpi!(externs::dpiVar_getSizeInBytes(self.inner, &mut size),
                 Ok(size),
                 ErrorKind::Var("dpiVar_getSizeInBytes".to_string()))
    }

    /// Releases a reference to the variable. A count of the references to the variable is
    /// maintained and when this count reaches zero, the memory associated with the variable is
    /// freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiVar_release(self.inner),
                 Ok(()),
                 ErrorKind::Var("dpiVar_release".to_string()))
    }

    /// Sets the variable value to the specified string. In the case of the variable's Oracle type
    /// being DPI_ORACLE_TYPE_NUMBER, the string is converted to an Oracle number during the call to
    /// this function.
    ///
    /// * `pos` - the array position in the variable which is to be set. The first position is 0. If
    /// the position exceeds the number of elements allocated by the variable an error is returned.
    /// * `value` - a string which contains the data to be set. The data is copied to the variable
    /// buffer and does not need to be retained after this function call has completed.
    pub fn set_from_bytes(&self, pos: u32, value: &str) -> Result<()> {
        let value_s = ODPIStr::from(value);
        try_dpi!(externs::dpiVar_setFromBytes(self.inner, pos, value_s.ptr(), value_s.len()),
                 Ok(()),
                 ErrorKind::Var("dpiVar_setFromBytes".to_string()))
    }

    /// Sets the variable value to the specified LOB.
    ///
    /// * `pos` - the array position in the variable which is to be set. The first position is 0. If
    /// the position exceeds the number of elements allocated by the variable an error is returned.
    /// * `lob` - the LOB which should be set.
    pub fn set_from_lob(&self, pos: u32, lob: Lob) -> Result<()> {
        try_dpi!(externs::dpiVar_setFromLob(self.inner, pos, lob.inner()),
                 Ok(()),
                 ErrorKind::Var("dpiVar_setFromLob".to_string()))
    }

    /// Sets the variable value to the specified object.
    ///
    /// * `pos` - the array position in the variable which is to be set. The first position is 0. If
    /// the position exceeds the number of elements allocated by the variable an error is returned.
    /// * `obj` - the object which should be set.
    pub fn set_from_object(&self, pos: u32, obj: Object) -> Result<()> {
        try_dpi!(externs::dpiVar_setFromObject(self.inner, pos, obj.inner()),
                 Ok(()),
                 ErrorKind::Var("dpiVar_setFromObject".to_string()))
    }

    /// Sets the variable value to the specified rowid.
    ///
    /// * `pos` - the array position in the variable which is to be set. The first position is 0. If
    /// the position exceeds the number of elements allocated by the variable an error is returned.
    /// * `rowid` - the rowid which should be set.
    pub fn set_from_rowid(&self, pos: u32, rowid: Rowid) -> Result<()> {
        try_dpi!(externs::dpiVar_setFromRowid(self.inner, pos, rowid.inner()),
                 Ok(()),
                 ErrorKind::Var("dpiVar_setFromRowid".to_string()))
    }

    /// Sets the variable value to the specified statement.
    ///
    /// * `pos` - the array position in the variable which is to be set. The first position is 0. If
    /// the position exceeds the number of elements allocated by the variable an error is returned.
    /// * `stmt` - the statement which should be set.
    pub fn set_from_stmt(&self, pos: u32, stmt: Statement) -> Result<()> {
        try_dpi!(externs::dpiVar_setFromStmt(self.inner, pos, stmt.inner()),
                 Ok(()),
                 ErrorKind::Var("dpiVar_setFromStmt".to_string()))
    }

    /// Sets the number of elements in a PL/SQL index-by table.
    ///
    /// * `num_elements` - he number of elements that PL/SQL should consider part of the array. This
    /// number should not exceed the number of elements that have been allocated in the variable.
    pub fn set_num_elements_in_array(&self, num_elements: u32) -> Result<()> {
        try_dpi!(externs::dpiVar_setNumElementsInArray(self.inner, num_elements),
                 Ok(()),
                 ErrorKind::Var("dpiVar_setNumElementsInArray".to_string()))
    }
}

impl From<*mut ODPIVar> for Var {
    fn from(inner: *mut ODPIVar) -> Var {
        Var { inner: inner }
    }
}
