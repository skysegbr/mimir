// Copyright (c) 2017 mimir developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! ODPI-C public structs.
use chrono::{DateTime, TimeZone, Utc};
use odpi::{enums, externs, flags, opaque};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use util::ODPIStr;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing application context to the database during the process of
/// creating standalone connections. These values are ignored when acquiring a connection from a
/// session pool or when using DRCP (Database Resident Connection Pooling). All values must be set
/// to valid values prior to being used in the `ODPIConnCreateParams` structure and must remain
/// valid until the execution of `Connection::create()` completes. Values set using this structure
/// are available in logon triggers by using the `sys_context()` SQL function.
pub struct ODPIAppContext {
    /// Specifies the value of the "namespace" parameter to `sys_context()`. It is expected to be a
    /// byte string in the encoding specified in the `ODPIConnCreateParams` structure and must not
    /// be NULL.
    pub namespace_name: *const c_char,
    /// Specifies the length of the `namespace_name` member, in bytes.
    pub namespace_name_length: u32,
    /// Specifies the value of the "parameter" parameter to `sys_context()`. It is expected to be a
    /// byte string in the encoding specified in the `ODPIConnCreateParams` structure and must not
    /// be NULL.
    pub name: *const c_char,
    /// Specifies the length of the `name` member, in bytes.
    pub name_length: u32,
    /// Specifies the value that will be returned from `sys_context()`. It is expected to be a byte
    /// string in the encoding specified in the `ODPIConnCreateParams` structure and must not be
    /// NULL.
    pub value: *const c_char,
    /// Specifies the length of the `value` member, in bytes.
    pub value_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing byte strings to and from the database in the structure
/// dpiData.
pub struct ODPIBytes {
    /// Specifies the pointer to the memory allocated by ODPI-C for the variable. For strings, data
    /// written to this memory should be in the encoding appropriate to the type of data being
    /// transferred. When data is transferred from the database it will be in the correct encoding
    /// already.
    pub ptr: *mut ::std::os::raw::c_char,
    /// Specifies the length of the byte string, in bytes.
    pub length: u32,
    /// Specifies the encoding for character data. This value is populated when data is transferred
    /// from the database. It is ignored when data is being transferred to the database.
    pub encoding: *const ::std::os::raw::c_char,
}

impl From<ODPIBytes> for String {
    fn from(odpi_bytes: ODPIBytes) -> String {
        let res_s = ODPIStr::new(odpi_bytes.ptr, odpi_bytes.length);
        res_s.into()
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used when creating session pools and standalone connections to the
/// database.
pub struct ODPICommonCreateParams {
    /// Specifies the mode used for creating connections. It is expected to be one or more of the
    /// values from the enumeration `ODPICreateMode`, OR'ed together. The default value is
    /// `DPI_MODE_CREATE_DEFAULT`.
    pub create_mode: flags::ODPICreateMode,
    /// Specifies the encoding to use for CHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_LANG environment variable. The default value is NULL.
    pub encoding: *const c_char,
    /// Specifies the encoding to use for NCHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_NCHAR environment variable. The default value is NULL.
    pub nchar_encoding: *const c_char,
    /// Specifies the edition to be used when creating a standalone connection. It is expected to
    /// be NULL (meaning that no edition is set) or a byte string in the encoding specified by the
    /// `encoding` member. The default value is NULL.
    pub edition: *const c_char,
    /// Specifies the length of the `edition` member, in bytes. The default value is 0.
    pub edition_length: u32,
    /// Specifies the name of the driver that is being used. It is expected to be NULL or a byte
    /// string in the encoding specified by the `encoding` member. The default value is NULL.
    pub driver_name: *const c_char,
    /// Specifies the length of the `driverName` member, in bytes. The default value is 0.
    pub driver_name_length: u32,
}

impl Default for ODPICommonCreateParams {
    fn default() -> ODPICommonCreateParams {
        ODPICommonCreateParams {
            create_mode: flags::DPI_MODE_CREATE_DEFAULT,
            encoding: ptr::null(),
            nchar_encoding: ptr::null(),
            edition: ptr::null(),
            edition_length: 0,
            driver_name: ptr::null(),
            driver_name_length: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for creating connections to the database, whether standalone or acquired
/// from a session pool. Care should be taken to ensure a copy of this structure exists only as long
/// as needed to create the connection since it can contain a clear text copy of credentials used
/// for connecting to the database.
pub struct ODPIConnCreateParams {
    /// Specifies the mode used for authorizing connections. It is expected to be one or more of the
    /// values from the enumeration `ODPIAuthMode`, OR'ed together. The default value is
    /// `DPI_MODE_AUTH_DEFAULT`.
    pub auth_mode: flags::ODPIAuthMode,
    /// Specifies the connection class to use when connecting to the database. This is used with
    /// DRCP (database resident connection pooling) or to further subdivide a session pool. It is
    /// expected to be NULL (meaning that no connection class will be set) or a byte string in the
    /// encoding used for CHAR data. The default value is NULL.
    pub connection_class: *const c_char,
    /// Specifies the length of the `connection_class` member, in bytes. The default value is 0.
    pub connection_class_length: u32,
    /// Specifies the level of purity required when creating a connection using a connection class.
    /// It is expected to be one of the values from the enumeration `ODPIPurity`. The default value
    /// is `ODPI_PURITY_DEFAULT`.
    pub purity: enums::ODPIPurity,
    /// Specifies the new password to set when creating a connection. This value is only used when
    /// creating a standalone connection. It is expected to be NULL or a byte string in the encoding
    /// used for CHAR data. The default value of this member is NULL. If specified, the password
    /// for the user is changed when the connection is created (useful when the password has expired
    /// and a session cannot be established without changing the password).
    pub new_password: *const c_char,
    /// Specifies the length of the `new_password` member, in bytes. The default value is 0.
    pub new_password_length: u32,
    /// Specifies the application context that will be set when the connection is created. This
    /// value is only used when creating standalone connections. It is expected to be NULL or an
    /// array of `ODPIAppContext` structures. The context specified here can be used in logon
    /// triggers, for example. The default value is NULL.
    pub app_context: *mut ODPIAppContext,
    /// Specifies the number of elements found in the `app_context` member. The default value is 0.
    pub num_app_context: u32,
    /// Specifies whether external authentication should be used to create the connection. If this
    /// value is 0, the user name and password values must be specified in the call to
    /// `Connection::create()`; otherwise, the user name and password values must be zero length or
    /// NULL. The default value is 0.
    pub external_auth: c_int,
    /// Specifies an OCI service context handle created externally that will be used instead of
    /// creating a connection. The default value is NULL.
    pub external_handle: *mut c_void,
    /// Specifies the session pool from which to acquire a connection or NULL if a standalone
    /// connection should be created. The default value is NULL.
    pub pool: *mut opaque::ODPIPool,
    /// Specifies the tag to use when acquiring a connection from a session pool. This member is
    /// ignored when creating a standalone connection. If specified, the tag restricts the type of
    /// session that can be returned to those with that tag or a NULL tag. If the member
    /// `match_any_tag` is set, however, any session can be returned if no matching sessions are
    /// found.
    ///
    /// The value is expected to be NULL (any session can be returned) or a byte string in the
    /// encoding used for CHAR data. The default value is NULL.
    pub tag: *const c_char,
    /// Specifies the length of the `tag` member, in bytes. The default value is 0.
    pub tag_length: u32,
    /// Specifies whether any tagged session should be accepted when acquiring a connection from a
    /// session pool, if no connection using the tag specified in the `tag` is available. This value
    /// is only used when acquiring a connection from a session pool. The default value is 0.
    pub match_any_tag: c_int,
    /// Specifies the tag of the connection that was acquired from a session pool, or NULL if the
    /// session was not tagged. This member is left untouched when creating a standalone connection
    /// and is filled in only if the connection acquired from the session pool was tagged. If filled
    /// in, it is a byte string in the encoding used for CHAR data.
    pub out_tag: *const c_char,
    /// Specifies the length of the `out_tag` member, in bytes.
    pub out_tag_length: u32,
    /// Specifies if the connection created used the tag specified by the `tag` member. It is only
    /// filled in if the connection was acquired from a session pool and a tag was initially
    /// specified.
    pub out_tag_found: c_int,
}

impl Default for ODPIConnCreateParams {
    fn default() -> ODPIConnCreateParams {
        ODPIConnCreateParams {
            auth_mode: flags::DPI_MODE_AUTH_DEFAULT,
            connection_class: ptr::null(),
            connection_class_length: 0,
            purity: enums::ODPIPurity::DefaultPurity,
            new_password: ptr::null(),
            new_password_length: 0,
            app_context: ptr::null_mut(),
            num_app_context: 0,
            external_auth: 0,
            external_handle: ptr::null_mut(),
            pool: ptr::null_mut(),
            tag: ptr::null(),
            tag_length: 0,
            match_any_tag: 0,
            out_tag: ptr::null(),
            out_tag_length: 0,
            out_tag_found: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
/// This structure is used for passing data to and from the database for variables and for
/// manipulating object attributes and collection values.
pub struct ODPIData {
    /// Specifies if the value refers to a null value (1) or not (0).
    pub is_null: ::std::os::raw::c_int,
    /// Specifies the value that is being passed or received.
    pub value: ODPIDataValueUnion,
}

impl Default for ODPIData {
    fn default() -> ODPIData {
        ODPIData {
            is_null: 1,
            value: ODPIDataValueUnion { as_boolean: 0 },
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(rustfmt, rustfmt_skip)]
/// Struct represention C union type for `ODPIData`.
pub union ODPIDataValueUnion {
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Boolean`. The value should be either 1 (true) or 0 (false).
    pub as_boolean: ::std::os::raw::c_int,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Int64`.
    pub as_int_64: i64,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Uint64`.
    pub as_uint_64: u64,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Float`.
    pub as_float: f32,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Double`.
    pub as_double: f64,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Bytes`. This is a structure of type `ODPIBytes`.
    pub as_bytes: ODPIBytes,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Timestamp`. This is a structure of type `ODPITimestamp`.
    pub as_timestamp: ODPITimestamp,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::IntervalDS`. This is a structure of type `ODPIIntervalDS`.
    pub as_interval_ds: ODPIIntervalDS,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::IntervalYM`. This is a structure of type `ODPIIntervalYM`.
    pub as_interval_ym: ODPIIntervalYM,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Lob`. This is a reference to a LOB (large object) which can be used for
    /// reading and writing the data that belongs to it.
    pub as_lob: *mut opaque::ODPILob,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Object`. This is a reference to an object which can be used for reading
    /// and writing its attributes or element values.
    pub as_object: *mut opaque::ODPIObject,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Stmt`. This is a reference to a statement which can be used to query
    /// data from the database.
    pub as_stmt: *mut opaque::ODPIStmt,
    /// Value that is used when `ODPIData.is_null` is 0 and the native type that is being used is
    /// `ODPINativeTypeNum::Rowid`. This is a reference to a rowid which is used to uniquely
    /// identify a row in a table in the database.
    pub as_rowid: *mut opaque::ODPIRowid,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// This structure is used for transferring encoding information from ODPI-C.
pub struct ODPIEncodingInfo {
    /// The encoding used for CHAR data, as a null-terminated ASCII string.
    pub encoding: *const ::std::os::raw::c_char,
    /// The maximum number of bytes required for each character in the encoding used for CHAR data.
    /// This value is used when calculating the size of buffers required when lengths in characters
    /// are provided.
    pub max_bytes_per_character: i32,
    /// The encoding used for NCHAR data, as a null-terminated ASCII string.
    pub nchar_encoding: *const ::std::os::raw::c_char,
    /// The maximum number of bytes required for each character in the encoding used for NCHAR data.
    /// Since this information is not directly available from Oracle it is only accurate if the
    /// encodings used for CHAR and NCHAR data are identical or one of ASCII or UTF-8; otherwise a
    /// value of 4 is assumed. This value is used when calculating the size of buffers required when
    /// lengths in characters are provided.
    pub nchar_max_bytes_per_character: i32,
}

impl Default for ODPIEncodingInfo {
    fn default() -> ODPIEncodingInfo {
        ODPIEncodingInfo {
            encoding: ptr::null(),
            max_bytes_per_character: 0,
            nchar_encoding: ptr::null(),
            nchar_max_bytes_per_character: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// This structure is used for transferring error information from ODPI-C. All of the strings
/// referenced here may become invalid as soon as the next ODPI-C call is made.
pub struct ODPIErrorInfo {
    /// The OCI error code if an OCI error has taken place. If no OCI error has taken place the
    /// value is 0.
    pub code: i32,
    /// The parse error offset (in bytes) when executing a statement or the row offset when fetching
    /// batch error information. If neither of these cases are true, the value is 0.
    pub offset: u16,
    /// The error message as a byte string in the encoding specified by the `encoding` member.
    pub message: *const c_char,
    /// The length of the `message` member, in bytes.
    pub message_length: u32,
    /// The encoding in which the error message is encoded as a null-terminated string. For OCI
    /// errors this is the CHAR encoding used when the connection was created. For ODPI-C specific
    /// errors this is UTF-8.
    pub encoding: *const c_char,
    /// The public ODPI-C function name which was called in which the error took place. This is a
    /// null-terminated ASCII string.
    pub fn_name: *const c_char,
    /// The internal action that was being performed when the error took place. This is a
    /// null-terminated ASCII string.
    pub action: *const c_char,
    /// The SQLSTATE code associated with the error. This is a 5 character null-terminated string.
    pub sql_state: *const c_char,
    /// A boolean value indicating if the error is recoverable. This member always has a value of 0
    /// unless both client and server are at release 12.1 or higher.
    pub is_recoverable: c_int,
}

impl Default for ODPIErrorInfo {
    fn default() -> ODPIErrorInfo {
        ODPIErrorInfo {
            code: 0,
            offset: 0,
            message: ptr::null(),
            message_length: 0,
            encoding: ptr::null(),
            fn_name: ptr::null(),
            action: ptr::null(),
            sql_state: ptr::null(),
            is_recoverable: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing interval (days to seconds) data to and from the database in
/// the structure dpiData.
pub struct ODPIIntervalDS {
    /// Specifies the number of days in the interval.
    pub days: i32,
    /// Specifies the number of hours in the interval.
    pub hours: i32,
    /// Specifies the number of minutes in the interval.
    pub minutes: i32,
    /// Specifies the number of seconds in the interval.
    pub seconds: i32,
    /// Specifies the number of fractional seconds in the interval (in nanoseconds).
    pub fseconds: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing interval (years to months) data to and from the database in
// the structure dpiData.
pub struct ODPIIntervalYM {
    /// Specifies the number of years in the interval.
    pub years: i32,
    /// Specifies the number of months in the interval.
    pub months: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information about an object type from ODPI-C. It is used by
/// the function `ObjectAttr::get_info()`.
pub struct ODPIObjectAttrInfo {
    /// Specifies the name of the attribute, as a byte string in the encoding used for CHAR data.
    pub name: *const ::std::os::raw::c_char,
    /// Specifies the length of the `name` member, in bytes.
    pub name_length: u32,
    /// Specifices the Oracle type of the attribute. It will be one of the values from the
    /// enumeration `ODPIOracleTypeNum`.
    pub oracle_type_num: enums::ODPIOracleTypeNum,
    /// Specifices the default native type of the attribute. It will be one of the values from the
    /// enumeration `ODPINativeTypeNum`.
    pub default_native_type_num: enums::ODPINativeTypeNum,
    /// Specifies a reference to the object type of the attribute, if the attribute refers to a
    /// named type; otherwise it is NULL.
    pub object_type: *mut opaque::ODPIObjectType,
}

impl Default for ODPIObjectAttrInfo {
    fn default() -> ODPIObjectAttrInfo {
        ODPIObjectAttrInfo {
            name: ptr::null(),
            name_length: 0,
            oracle_type_num: enums::ODPIOracleTypeNum::Max,
            default_native_type_num: enums::ODPINativeTypeNum::Invalid,
            object_type: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information about an object type from ODPI-C. It is used by
/// the function `ObjectType::getInfo()`.
pub struct ODPIObjectTypeInfo {
    /// Specifies the schema which owns the object type, as a byte string in the encoding used for
    /// CHAR data.
    pub schema: *const ::std::os::raw::c_char,
    /// Specifies the length of the `schema` member, in bytes.
    pub schema_length: u32,
    /// Specifies the name of the object type, as a byte string in the encoding used for CHAR data.
    pub name: *const ::std::os::raw::c_char,
    /// Specifies the length of the `name` member, in bytes.
    pub name_length: u32,
    /// Specifies if the object type is a collection (1) or not (0).
    pub is_collection: ::std::os::raw::c_int,
    /// Specifies the Oracle type of the elements in the collection if the object type refers to a
    /// collection. It will be one of the values from the enumeration `ODPIOracleTypeNum`.
    pub element_oracle_type_num: enums::ODPIOracleTypeNum,
    /// Specifies the default native type of the elements in the collection if the object type
    /// refers to a collection. It will be one of the values from the enumeration
    /// `ODPINativeTypeNum`.
    pub element_default_native_type_num: enums::ODPINativeTypeNum,
    /// Specifies a reference to the object type of the elements in the collection if the object
    /// type on which info is being returned refers to a collection.
    pub element_object_type: *mut opaque::ODPIObjectType,
    /// Specifies the number of attributes that the object type has.
    pub num_attributes: u16,
}

impl Default for ODPIObjectTypeInfo {
    fn default() -> ODPIObjectTypeInfo {
        ODPIObjectTypeInfo {
            schema: ptr::null(),
            schema_length: 0,
            name: ptr::null(),
            name_length: 0,
            is_collection: 0,
            element_oracle_type_num: enums::ODPIOracleTypeNum::Max,
            element_default_native_type_num: enums::ODPINativeTypeNum::Invalid,
            element_object_type: ptr::null_mut(),
            num_attributes: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for creating session pools, which can in turn be used to create
/// connections that are acquired from that session pool.
pub struct ODPIPoolCreateParams {
    /// Specifies the minimum number of sessions to be created by the session pool. This value is
    /// ignored if the `homogeneous` member has a value of 0. The default value is 1.
    pub min_sessions: u32,
    /// Specifies the maximum number of sessions that can be created by the session pool. Values of
    /// 1 and higher are acceptable. The default value is 1.
    pub max_sessions: u32,
    /// Specifies the number of sessions that will be created by the session pool when more sessions
    /// are required and the number of sessions is less than the maximum allowed. This value is
    /// ignored if the `homogeneous` member has a value of 0. This value added to the `minSessions`
    /// member value must not exceed the `maxSessions` member value. The default value is 0.
    pub session_increment: u32,
    /// Specifies the number of seconds since a connection has last been used before a ping will be
    /// performed to verify that the connection is still valid. A negative value disables this
    /// check. The default value is 60. This value is ignored in clients 12.2 and later since a much
    /// faster internal check is done by the Oracle client.
    pub ping_interval: c_int,
    /// Specifies the number of milliseconds to wait when performing a ping to verify the connection
    /// is still valid before the connection is considered invalid and is dropped. The default value
    /// is 5000 (5 seconds). This value is ignored in clients 12.2 and later since a much faster
    /// internal check is done by the Oracle client.
    pub ping_timeout: c_int,
    /// Specifies whether the pool is homogeneous or not. In a homogeneous pool all connections use
    /// the same credentials whereas in a heterogeneous pool other credentials are permitted. The
    /// default value is 1.
    pub homogeneous: c_int,
    /// Specifies whether external authentication should be used to create the sessions in the pool.
    /// If this value is 0, the user name and password values must be specified in the call to
    /// `Pool::create()`; otherwise, the user name and password values must be zero length or NULL.
    /// The default value is 0.
    pub external_auth: c_int,
    /// Specifies the mode to use when sessions are acquired from the pool. It is expected to be one
    /// of the values from the enumeration `ODPIGetPoolMode`. The default value is
    /// DPI_MODE_POOL_GET_NOWAIT
    pub get_mode: enums::ODPIPoolGetMode,
    /// This member is populated upon successful creation of a pool using the function
    /// `Pool::create()`. It is a byte string in the encoding used for CHAR data. Any value
    /// specified prior to creating the session pool is ignored.
    pub out_pool_name: *const c_char,
    /// This member is populated upon successful creation of a pool using the function
    /// `Pool::create()`. It is the length of the `out_pool_name` member, in bytes. Any value
    /// specified prior to creating the session pool is ignored.
    pub out_pool_name_length: u32,
}

impl Default for ODPIPoolCreateParams {
    fn default() -> ODPIPoolCreateParams {
        ODPIPoolCreateParams {
            min_sessions: 0,
            max_sessions: 1,
            session_increment: 0,
            ping_interval: 60,
            ping_timeout: 5000,
            homogeneous: 0,
            external_auth: 0,
            get_mode: enums::ODPIPoolGetMode::NoWait,
            out_pool_name: ptr::null(),
            out_pool_name_length: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing query metadata from ODPI-C. It is populated by the function
/// `Statement::getQueryInfo()`. All values remain valid as long as a reference is held to the
/// statement and the statement is not re-executed or closed.
pub struct ODPIQueryInfo {
    /// Specifies the name of the column which is being queried, as a byte string in the encoding
    /// used for CHAR data.
    pub name: *const ::std::os::raw::c_char,
    /// Specifies the length of the `name` member, in bytes.
    pub name_length: u32,
    /// Specifies the type of the column that is being queried. It will be one of the values from
    /// the enumeration `ODPIOracleTypeNum`.
    pub oracle_type_num: enums::ODPIOracleTypeNum,
    /// Specifies the default native type for the column that is being queried. It will be one of
    /// the values from the enumeration `ODPINativeTypeNum`.
    pub default_native_type_num: enums::ODPINativeTypeNum,
    /// Specifies the size in bytes (from the database's perspective) of the column that is being
    /// queried. This value is only populated for strings and binary columns. For all other columns
    /// the value is zero.
    pub db_size_in_bytes: u32,
    /// Specifies the size in bytes (from the client's perspective) of the column that is being
    /// queried. This value is only populated for strings and binary columns. For all other columns
    /// the value is zero.
    pub client_size_in_bytes: u32,
    /// Specifies the size in characters of the column that is being queried. This value is only
    /// populated for string columns. For all other columns the value is zero.
    pub size_in_chars: u32,
    /// Specifies the precision of the column that is being queried. This value is only populated
    /// for numeric and timestamp columns. For all other columns the value is zero.
    pub precision: i16,
    /// Specifies the scale of the column that is being queried. This value is only populated for
    /// numeric columns. For all other columns the value is zero.
    pub scale: i8,
    /// Specifies if the column that is being queried may return null values (1) or not (0).
    pub null_ok: ::std::os::raw::c_int,
    /// Specifies a reference to the type of the object that is being queried. This value is only
    /// populated for named type columns. For all other columns the value is NULL. The reference
    /// that is returned must be released when it is no longer needed.
    pub object_type: *mut opaque::ODPIObjectType,
}

impl Default for ODPIQueryInfo {
    fn default() -> ODPIQueryInfo {
        ODPIQueryInfo {
            name: ptr::null(),
            name_length: 0,
            oracle_type_num: enums::ODPIOracleTypeNum::TypeNone,
            default_native_type_num: enums::ODPINativeTypeNum::Invalid,
            db_size_in_bytes: 0,
            client_size_in_bytes: 0,
            size_in_chars: 0,
            precision: 0,
            scale: 0,
            null_ok: 0,
            object_type: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information about a statement from ODPI-C. It is used by the
/// function `Statement::getInfo()`.
pub struct ODPIStmtInfo {
    /// Specifies if the statement refers to a query (1) or not (0).
    pub is_query: ::std::os::raw::c_int,
    /// Specifies if the statement refers to a PL/SQL block (1) or not (0).
    pub is_plsql: ::std::os::raw::c_int,
    /// Specifies if the statement refers to DDL (data definition language) such as creating a table
    /// (1) or not (0).
    pub is_ddl: ::std::os::raw::c_int,
    /// Specifies if the statement refers to DML (data manipulation language) such as inserting,
    /// updating and deleting (1) or not (0).
    pub is_dml: ::std::os::raw::c_int,
    /// Specifies the type of statement that has been prepared. The members `is_query`, `is_plsql`,
    /// `is_ddl` and `is_dml` are all categorizations of this value. It will be one of the values
    /// from the enumeration `ODPIStatementType`.
    pub statement_type: enums::ODPIStatementType,
    /// Specifies if the statement has a returning clause in it (1) or not (0).
    pub is_returning: ::std::os::raw::c_int,
}

impl Default for ODPIStmtInfo {
    fn default() -> ODPIStmtInfo {
        ODPIStmtInfo {
            is_query: 0,
            is_plsql: 0,
            is_ddl: 0,
            is_dml: 0,
            statement_type: enums::ODPIStatementType::NotSet,
            is_returning: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for creating subscriptions to messages sent for object change
/// notification, query change notification or advanced queuing.
pub struct ODPISubscrCreateParams {
    /// Specifies the namespace in which the subscription is created. It is expected to be one of
    /// the values from the enumeration `ODPISubscrNamespace`. The default value is
    /// DPI_SUBSCR_NAMESPACE_DBCHANGE.
    pub subscr_namespace: enums::ODPISubscrNamespace,
    /// Specifies the protocol used for sending notifications for the subscription. It is expected
    /// to be one of the values from the enumeration `ODPISubscrProtocol`. The default value is
    /// DPI_SUBSCR_PROTO_CALLBACK.
    pub protocol: enums::ODPISubscrProtocol,
    /// Specifies the quality of service flags to use with the subscription. It is expected to be
    /// one or more of the values from the enumeration `ODPISubscrQOS`, OR'ed together. The default
    /// value is to have no flags set.
    pub qos: flags::ODPISubscrQOS,
    /// Specifies which operations on the registered tables or queries should result in
    /// notifications. It is expected to be one or more of the values from the enumeration
    /// `ODPIOpCode`, OR'ed together. The default value is DPI_OPCODE_ALL_OPS.
    pub operations: flags::ODPIOpCode,
    /// Specifies the port number on which to receive notifications. The default value is 0, which
    /// means that a port number will be selected by the Oracle client.
    pub port_number: u32,
    /// Specifies the length of time, in seconds, before the subscription is unregistered. If the
    /// value is 0, the subscription remains active until explicitly unregistered. The default value
    /// is 0.
    pub timeout: u32,
    /// Specifies the name of the subscription, as a byte string in the encoding used for CHAR data.
    /// This name must be consistent with the namespace identified in the `subscr_namespace` member.
    /// The default value is NULL.
    pub name: *const c_char,
    /// Specifies the length of the `name` member, in bytes. The default value
    /// is 0.
    pub name_length: u32,
    /// Specifies the callback that will be called when a notification is sent to the subscription,
    /// if the `protocol` member is set to DPI_SUBSCR_PROTO_CALLBACK. The callback accepts the
    /// following arguments:
    ///
    /// * context -- the value of the `callback_context` member.
    /// * message -- a pointer to the message that is being sent. The message is in the form
    ///              `ODPISubscrMessage`.
    ///
    /// The default value is NULL. If a callback is specified and a notification is sent, this will
    /// be performed on a separate thread. If database operations are going to take place, ensure
    /// that the create mode DPI_MODE_CREATE_THREADED is set in the structure dpiCommonCreateParams
    /// when creating the session pool or standalone connection that will be used in this callback.
    pub callback: externs::ODPISubscrCallback,
    /// Specifies the value that will be used as the first argument to the callback specified in the
    /// `callback` member. The default value is NULL.
    pub callback_context: *mut c_void,
    /// Specifies the name of the recipient to which notifications are sent when the
    /// dpiSubscrCreateParams.protocol member is not set to DPI_SUBSCR_PROTO_CALLBACK. The value is
    /// expected to be a byte string in the encoding used for CHAR data. The default value is NULL.
    pub recipient_name: *const c_char,
    /// Specifies the length of the `recipient_name` member, in bytes. The default value is 0.
    pub recipient_name_length: u32,
}

impl Default for ODPISubscrCreateParams {
    fn default() -> ODPISubscrCreateParams {
        ODPISubscrCreateParams {
            subscr_namespace: enums::ODPISubscrNamespace::DbChange,
            protocol: enums::ODPISubscrProtocol::Callback,
            qos: flags::DPI_SUBSCR_QOS_NONE,
            operations: flags::DPI_OPCODE_ALL_OPS,
            port_number: 0,
            timeout: 0,
            name: ptr::null(),
            name_length: 0,
            callback: None,
            callback_context: ptr::null_mut(),
            recipient_name: ptr::null(),
            recipient_name_length: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing messages sent by notifications to subscriptions. It is the
/// second parameter to the callback method specified in the `ODPISubscrCreateParams` structure.
pub struct ODPISubscrMessage {
    /// Specifies the type of event that took place which generated the notification. It will be one
    /// of the values from the enumeration `ODPIEventType`.
    pub event_type: enums::ODPIEventType,
    /// Specifies the name of the database which generated the notification, as a byte string in the
    /// encoding used for CHAR data.
    pub db_name: *const c_char,
    /// Specifies the length of the `db_name` member, in bytes.
    pub db_name_length: u32,
    /// Specifies a pointer to an array of `ODPISubscrMessageTable` structures representing the list
    /// of tables that were modified and generated this notification. This value will be NULL if the
    /// value of the `event_type` member is not equal to DPI_EVENT_OBJCHANGE.
    pub tables: *mut ODPISubscrMessageTable,
    /// Specifies the number of structures available in the `tables` member.
    pub num_tables: u32,
    /// Specifies a pointer to an array of `ODPISubscrMessageQuery` structures representing the list
    /// of queries that were modified and generated this notification. This value will be NULL if
    /// the value of the `event_type` member is not equal to DPI_EVENT_QUERYCHANGE.
    pub queries: *mut ODPISubscrMessageQuery,
    /// Specifies the number of structures available in the `queries` member.
    pub num_queries: u32,
    /// Specifies a pointer to a `ODPIErrorInfo` structure. This value will be NULL if no error has
    /// taken place. If this value is not NULL the other members in this structure may not contain
    /// valid values.
    pub error_info: *mut ODPIErrorInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information on query change notification events and is part
/// of the `ODPISubscrMessage` structure.
pub struct ODPISubscrMessageQuery {
    /// Specifies the id of the query that was registered as part of the subscription that generated
    /// this notification.
    pub id: u64,
    /// Specifies the operations that took place on the registered query. It will be one or more of
    /// the values from the enumeration `ODPIOpCode`, OR'ed together.
    pub operation: flags::ODPIOpCode,
    /// Specifies a pointer to an array of `ODPISubscrMessageTable` structures representing the list
    /// of tables that were modified by the event which generated this notification.
    pub tables: *mut ODPISubscrMessageTable,
    /// Specifies the number of structures available in the `tables` member.
    pub num_tables: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information on the rows that were changed and resulted in the
/// notification message of which this structure is a part.
pub struct ODPISubscrMessageRow {
    /// Specifies the operations that took place on the registered query. It will be one or more of
    /// the values from the enumeration `ODPIOpCode`, OR'ed together.
    pub operation: flags::ODPIOpCode,
    /// Specifies the rowid of the row that was changed, in the encoding used for CHAR data.
    pub rowid: *const c_char,
    /// Specifies the length of the `rowid` member, in bytes.
    pub rowid_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information on the tables that were changed and resulted in
/// the notification message of which this structure is a part.
pub struct ODPISubscrMessageTable {
    /// Specifies the operations that took place on the modified table. It will be one or more of
    /// the values from the enumeration `ODPIOpCode`, OR'ed together.
    pub operation: flags::ODPIOpCode,
    /// Specifies the name of the table that was changed, in the encoding used for CHAR data.
    pub name: *const c_char,
    /// Specifies the length of the `name` member, in bytes.
    pub name_length: u32,
    /// Specifies a pointer to an array of `ODPISubscrMessageRow` structures representing the list
    /// of rows that were modified by the event which generated this notification.
    pub rows: *mut ODPISubscrMessageRow,
    /// Specifies the number of structures available in the `rows` member.
    pub num_rows: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
/// This structure is used for passing timestamp data to and from the database in the structure
/// dpiData.
pub struct ODPITimestamp {
    /// Specifies the year for the timestamp.
    pub year: i16,
    /// Specifies the month for the timestamp. This should be between 1 and 12.
    pub month: u8,
    /// Specifies the day for the timestamp. This should be between 1 and 31.
    pub day: u8,
    /// Specifies the hour for the timestamp. This should be between 0 and 23.
    pub hour: u8,
    /// Specifies the minute for the timestamp. This should be between 0 and 59.
    pub minute: u8,
    /// Specifies the second for the timestamp. This should be between 0 and 59.
    pub second: u8,
    /// Specifies the fractional seconds for the timestamp, in nanoseconds.
    pub fsecond: u32,
    /// Specifies the hours offset from UTC. This value is only used for timestamp with time zone
    /// and timestamp with local time zone columns.
    pub tz_hour_offset: i8,
    /// Specifies the minutes offset from UTC. This value is only used for timestamp with time zone
    /// and timestamp with local time zone columns.
    pub tz_minute_offset: i8,
}

impl From<ODPITimestamp> for DateTime<Utc> {
    fn from(timestamp: ODPITimestamp) -> DateTime<Utc> {
        let y = timestamp.year as i32;
        let m = timestamp.month as u32;
        let d = timestamp.day as u32;
        let h = timestamp.hour as u32;
        let mm = timestamp.minute as u32;
        let s = timestamp.second as u32;
        let fs = timestamp.fsecond * 1000;

        if y == -10100 && m == 0 && d == 0 {
            Utc::now()
        } else {
            Utc.ymd(y, m, d).and_hms_micro(h, mm, s, fs)
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for returning information about the Oracle Client.
pub struct ODPIVersionInfo {
    /// Specifies the major version of the Oracle Client or Database.
    pub version_num: c_int,
    /// Specifies the release version of the Oracle Client or Database.
    pub release_num: c_int,
    /// Specifies the update version of the Oracle Client or Database.
    pub update_num: c_int,
    /// Specifies the port specific release version of the Oracle Client or Database.
    pub port_release_num: c_int,
    /// Specifies the port specific update version of the Oracle Client or Database.
    pub port_update_num: c_int,
    /// Specifies the full version (all five components) as a number that is suitable for
    /// comparison with the result of the macro DPI_ORACLE_VERSION_TO_NUMBER.
    pub full_version_num: u32,
}

impl Default for ODPIVersionInfo {
    fn default() -> ODPIVersionInfo {
        ODPIVersionInfo {
            version_num: 0,
            release_num: 0,
            update_num: 0,
            port_release_num: 0,
            port_update_num: 0,
            full_version_num: 0,
        }
    }
}
