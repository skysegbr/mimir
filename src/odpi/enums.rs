// Copyright (c) 2017 mimir developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! ODPI-C public enums
use std::fmt;

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the modes that are possible when dequeuing messages from a queue.
pub enum ODPIDeqMode {
    /// Read the message without acquiring a lock on the message (equivalent to a SELECT statement).
    Browse = 1,
    /// Read the message and obtain a write lock on the message (equivalent to a SELECT FOR UPDATE
    /// statement).
    Locked = 2,
    /// Read the message and update or delete it. This is the default mode. Note that the message
    /// may be retained in the queue table based on retention properties.
    Remove = 3,
    /// Confirms receipt of the message but does not deliver the actual message content.
    RemoveNoData = 4,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the method used for determining which message is to be dequeued from
/// a queue.
pub enum ODPIDeqNavigation {
    /// Retrieves the first available message that matches the search criteria. This resets the
    /// position to the beginning of the queue.
    FirstMsg = 1,
    /// Skips the remainder of the current transaction group (if any) and retrieves the first
    /// message of the next transaction group. This option can only be used if message grouping is
    /// enabled for the queue.
    NextTransaction = 2,
    /// Retrieves the next available message that matches the search criteria. This is the default
    /// method.
    NextMsg = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the types of events that can take place. The event type is part of
/// the messages that are sent to subscriptions.
pub enum ODPIEventType {
    /// Indicates that no event has taken place.
    No = 0,
    /// Indicates that a database is being started up.
    Startup = 1,
    /// Indicates that a database is being shut down.
    Shutdown = 2,
    /// Indicates that a database is being shut down.
    ShutdownAny = 3,
    /// Indicates that a database is being dropped.
    DropDB = 4,
    /// Indicates that a registered query is being deregistered.
    Dereg = 5,
    /// Indicates that an object change has taken place.
    ObjectChange = 6,
    /// Indicates that a query change has taken place.
    QueryChange = 7,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the mode to use when scrolling the cursor to a new location using
/// the function `Statement::scroll()`.
pub enum ODPIFetchMode {
    /// Scroll the cursor to the next row in the result set. The offset is ignored when using this
    /// mode.
    Next = 2,
    /// Scroll the cursor to the first row in the result set. The offset is ignored when using this
    /// mode.
    First = 4,
    /// Scroll the cursor to the last row in the result set. The offset is ignored when using this
    /// mode.
    Last = 8,
    /// Scroll the cursor to the previous row in the result set. The offset is ignored when using
    /// this mode.
    Prior = 16,
    /// Scroll the cursor to the row identified by the offset parameter using absolute positioning.
    Absolte = 32,
    /// Scroll the cursor to the row identified by the offset parameter using relative positioning.
    /// A positive number will move forward in the result set while a negative number will move
    /// backwards in the result set.
    Relative = 64,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the delivery mode used for filtering messages when dequeuing
/// messages from a queue.
pub enum ODPIMessageDeliveryMode {
    /// Not set.
    NotSet = 0,
    /// Dequeue only persistent messages from the queue. This is the default mode.
    Persistent = 1,
    /// Dequeue only buffered messages from the queue.
    Buffered = 2,
    /// Dequeue both persistent and buffered messages from the queue.
    PersistentOrBuffered = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the possible states for messages in a queue.
pub enum ODPIMessageState {
    /// The message is ready to be processed.
    Ready = 0,
    /// The message is waiting for the delay time to expire.
    Waiting = 1,
    /// The message has already been processed and is retained.
    Processed = 2,
    /// The message has been moved to the exception queue.
    Expired = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the type of data that is being transferred to and from the database.
pub enum ODPINativeTypeNum {
    /// An invalid native type num.
    Invalid = 0,
    /// Data is passed as a 64-bit integer in the asInt64 member of dpiData.value.
    Int64 = 3000,
    /// Data is passed as an unsigned 64-bit integer in the asUint64 member of dpiData.value.
    Uint64 = 3001,
    /// Data is passed as a single precision floating point number in the asFloat member of
    /// dpiData.value.
    Float = 3002,
    /// Data is passed as a double precision floating point number in the asDouble member of
    /// dpiData.value.
    Double = 3003,
    /// Data is passed as a byte string in the asBytes member of dpiData.value.
    Bytes = 3004,
    /// Data is passed as a timestamp in the asTimestamp member of dpiData.value.
    Timestamp = 3005,
    /// Data is passed as an interval (days to seconds) in the asIntervalDS member of dpiData.value.
    IntervalDS = 3006,
    /// Data is passed as an interval (years to months) in the asIntervalYM member of dpiData.value.
    IntervalYM = 3007,
    /// Data is passed as a reference to a LOB in the asLOB member of dpiData.value.
    Lob = 3008,
    /// Data is passed as a reference to an object in the asObject member of dpiData.value.
    Object = 3009,
    /// Data is passed as a reference to a statement in the asStmt member of dpiData.value.
    Stmt = 3010,
    /// Data is passed as a boolean value in the asBoolean member of dpiData.value.
    Boolean = 3011,
    /// Data is passed as a reference to a rowid in the asRowid member of dpiData.value.
    Rowid = 3012,
}

impl From<i32> for ODPINativeTypeNum {
    fn from(val: i32) -> ODPINativeTypeNum {
        match val {
            3000 => ODPINativeTypeNum::Int64,
            3001 => ODPINativeTypeNum::Uint64,
            3002 => ODPINativeTypeNum::Float,
            3003 => ODPINativeTypeNum::Double,
            3004 => ODPINativeTypeNum::Bytes,
            3005 => ODPINativeTypeNum::Timestamp,
            3006 => ODPINativeTypeNum::IntervalDS,
            3007 => ODPINativeTypeNum::IntervalYM,
            3008 => ODPINativeTypeNum::Lob,
            3009 => ODPINativeTypeNum::Object,
            3010 => ODPINativeTypeNum::Stmt,
            3011 => ODPINativeTypeNum::Boolean,
            3012 => ODPINativeTypeNum::Rowid,
            _ => ODPINativeTypeNum::Invalid,
        }
    }
}

impl fmt::Display for ODPINativeTypeNum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let desc = match *self {
            ODPINativeTypeNum::Int64 => "Int64",
            ODPINativeTypeNum::Uint64 => "Uint64",
            ODPINativeTypeNum::Float => "Float",
            ODPINativeTypeNum::Double => "Double",
            ODPINativeTypeNum::Bytes => "Bytes",
            ODPINativeTypeNum::Timestamp => "Timestamp",
            ODPINativeTypeNum::IntervalDS => "IntervalDS",
            ODPINativeTypeNum::IntervalYM => "IntervalYM",
            ODPINativeTypeNum::Lob => "LOB",
            ODPINativeTypeNum::Object => "Object",
            ODPINativeTypeNum::Stmt => "Stmt",
            ODPINativeTypeNum::Boolean => "Boolean",
            ODPINativeTypeNum::Rowid => "Rowid",
            ODPINativeTypeNum::Invalid => "Invalid",
        };
        write!(f, "{}", desc)
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the types of Oracle data that can be used for binding data as
/// arguments to a statement, fetching data from the database, or getting and setting object
/// attributes and element values.
pub enum ODPIOracleTypeNum {
    /// None type.
    TypeNone = 2000,
    /// Default type used for VARCHAR2 columns in the database. Data is transferred to/from Oracle
    /// as byte strings in the encoding used for CHAR data.
    Varchar = 2001,
    /// Default type used for NVARCHAR2 columns in the database. Data is transferred to/from Oracle
    /// as byte strings in the encoding used for NCHAR data.
    NVarchar = 2002,
    /// Default type used for CHAR columns in the database. Data is transferred to/from Oracle as
    /// byte strings in the encoding used for CHAR data.
    Char = 2003,
    /// Default type used for NCHAR columns in the database. Data is transferred to/from Oracle as
    /// byte strings in the encoding used for NCHAR data.
    NChar = 2004,
    /// Default type used for the pseudocolumn "ROWID". Data is transferred to/from Oracle as byte
    /// strings, in the encoding used for CHAR data.
    RowID = 2005,
    /// Default type used for RAW columns in the database. Data is transferred to/from Oracle as raw
    /// byte strings.
    Raw = 2006,
    /// Default type used for BINARY_FLOAT columns in the database. Data is transferred to/from
    /// Oracle as the C float type.
    NativeFloat = 2007,
    /// Default type used for BINARY_DOUBLE columns in the database. Data is transferred to/from
    /// Oracle as the C double type.
    NativeDouble = 2008,
    /// Type available for binding native integers directly in PL/SQL (such as PLS_INTEGER). Data is
    /// transferred to/from Oracle as 64-bit integers.
    NativeInt = 2009,
    /// Default type used for NUMBER columns in the database. Data is transferred to/from Oracle in
    /// Oracle's internal format.
    Number = 2010,
    ///	Default type used for DATE columns in the database. Data is transferred to/from Oracle in
    /// Oracle's internal format.
    Date = 2011,
    /// Default type used for TIMESTAMP columns in the database. Data is transferred to/from Oracle
    /// in Oracle's internal format.
    Timestamp = 2012,
    /// Default type used for TIMESTAMP WITH TIME ZONE columns in the database. Data is transferred
    /// to/from Oracle in Oracle's internal format.
    TimestampTz = 2013,
    /// Default type used for TIMESTAMP WITH LOCAL TIME ZONE columns in the database. Data is
    /// transferred to/from Oracle in Oracle's internal format.
    TimestampLtz = 2014,
    /// Default type used for INTERVAL DAY TO SECOND columns in the database. Data is transferred
    /// to/from Oracle in Oracle's internal format.
    IntervalDS = 2015,
    /// Default type used for INTERVAL YEAR TO MONTH columns in the database. Data is transferred
    /// to/from Oracle in Oracle's internal format.
    IntervalYM = 2016,
    /// Default type used for CLOB columns in the database. Only a locator is transferred to/from
    /// Oracle, which can subsequently be used via dpiLob references to read/write from that
    /// locator.
    Clob = 2017,
    /// Default type used for NCLOB columns in the database. Only a locator is transferred to/from
    /// Oracle, which can subsequently be used via dpiLob references to read/write from that
    /// locator.
    NClob = 2018,
    /// Default type used for BLOB columns in the database. Only a locator is transferred to/from
    /// Oracle, which can subsequently be used via dpiLob references to read/write from that
    /// locator.
    Blob = 2019,
    /// Default type used for BFILE columns in the database. Only a locator is transferred to/from
    /// Oracle, which can subsequently be used via dpiLob references to read/write from that
    /// locator.
    BFile = 2020,
    /// Used within PL/SQL for REF CURSOR or within SQL for querying a CURSOR. Only a handle is
    /// transferred to/from Oracle, which can subsequently be used via dpiStmt for querying.
    Stmt = 2021,
    /// Used within PL/SQL for boolean values. This is only available in 12.1. Earlier releases
    /// simply use the integer values 0 and 1 to represent a boolean value. Data is transferred
    /// to/from Oracle as an integer.
    Boolean = 2022,
    /// Default type used for named type columns in the database. Data is transferred to/from Oracle
    /// in Oracle's internal format.
    Object = 2023,
    /// Default type used for LONG columns in the database. Data is transferred to/from Oracle as
    /// byte strings in the encoding used for CHAR data.
    LongVarchar = 2024,
    /// Default type used for LONG RAW columns in the database. Data is transferred to/from Oracle
    /// as raw byte strings.
    LongRaw = 2025,
    /// Type available for binding native integers directly in PL/SQL (such as PLS_INTEGER). Data is
    /// transferred to/from Oracle as 64-bit unsigned integers.
    NativeUint = 2026,
    /// Max Type.
    Max = 2027,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the mode to use when getting sessions from a session pool.
pub enum ODPIPoolGetMode {
    /// Specifies that the caller should block until a session is available from the pool.
    Wait,
    /// Specifies that the caller should return immediately, regardless of whether a session is
    /// available in the pool. If a session is not available an error is returned.
    NoWait,
    /// Specifies that a new session should be created if all of the sessions in the pool are  busy,
    /// even if this exceeds the maximum sessions allowable for the session pool (see
    /// `ODPIPoolCreateParams.maxSessions`)
    ForceGet,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the purity of the sessions that are acquired when using
/// connection classes during connection creation.
pub enum ODPIPurity {
    /// Default value used when creating connections.
    DefaultPurity = 0,
    /// A connection is required that has not been tainted with any prior session state.
    New = 1,
    /// A connection is permitted to have prior session state.
    SelfPurity = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the mode to use when shutting down a database using
/// `dpiConn_shutdownDatabase()`.
pub enum ODPIShutdownMode {
    /// Further connections to the database are prohibited. Wait for users to disconnect from the
    /// database.
    DefaultShutdownMode = 0,
    /// Further connections to the database are prohibited and no new transactions are allowed to be
    /// started. Wait for active transactions to complete.
    Transactional = 1,
    /// Behaves the same way as `Transactional` but only waits for local transactions to complete.
    TransactionalLocal = 2,
    /// All uncommitted transactions are terminated and rolled back and all connections to the
    /// database are closed immediately.
    Immediate = 3,
    /// All uncommitted transactions are terminated and are not rolled back. This is the fastest way
    /// to shut down the database but the next database startup may require instance recovery.
    Abort = 4,
    /// Shuts down the database. This mode should only be used in the second call to
    /// dpiConn_shutdownDatabase().
    Final = 5,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the mode to use when starting up a database using
/// `dpiConn_startupDatabase()`.
pub enum ODPIStartupMode {
    /// Default mode for startup which permits database access to all users.
    DefaultStartupMode = 0,
    /// Shuts down a running instance (using ABORT) before starting a new one. This mode should only
    /// be used in unusual circumstances.
    Force = 1,
    /// Only allows database access to users with both the CREATE SESSION and RESTRICTED SESSION
    /// privileges (normally the DBA).
    Restrict = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the type of statement that has been prepared. It is available as
/// part of the structure `ODPIStmtInfo`.
pub enum ODPIStatementType {
    /// Statement type not set.
    NotSet = 0,
    /// Identifies a select statement. The member `ODPIStmtInfo.is_query` will be set to 1.
    Select = 1,
    /// Identifies an update statement. The member `ODPIStmtInfo.is_dml` will be set to 1.
    Update = 2,
    /// Identifies a delete statement. The member `ODPIStmtInfo.is_dml` will be set to 1.
    Delete = 3,
    /// Identifies an insert statement. The member `ODPIStmtInfo.is_dml` will be set to 1.
    Insert = 4,
    /// Identifies a create statement. The member `ODPIStmtInfo.is_ddl` will be set to 1.
    Create = 5,
    /// Identifies a drop statement. The member `ODPIStmtInfo.is_ddl` will be set to 1.
    DropDdl = 6,
    /// Identifies an alter statement. The member `ODPIStmtInfo.is_ddl` will be set to 1.
    Alter = 7,
    /// Identifies an anonymous PL/SQL block starting with the keyword begin. The member
    /// `ODPIStmtInfo.is_plsql will be set to 1.
    Begin = 8,
    /// Identifies an anonymous PL/SQL block starting with the keyword declare. The member
    /// `ODPIStmtInfo.is_plsql` will be set to 1.
    Declare = 9,
    /// Identifies a CALL statement used for calling stored procedures and functions. The member
    /// `ODPIStmtInfo.is_plsql` will be set to 1.
    Call = 10,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the namespaces supported by subscriptions.
pub enum ODPISubscrNamespace {
    /// Identifies the namespace used for receiving notifications for database object changes
    /// and query changes.
    DbChange = 0,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the protocol used for sending notifications to subscriptions.
pub enum ODPISubscrProtocol {
    /// Notifications are sent by calling the callback specified when the subscription was
    /// registered.
    Callback = 0,
    /// Notifications are sent by sending an e-mail to the e-mail address specified when the
    /// subscription was registered.
    Mail = 1,
    /// Notifications are sent by calling the PL/SQL procedure specified when the subscription was
    /// registered.
    PLSQL = 2,
    /// Notifications are sent to the URL specified when the subscription was registered.
    HTTP = 3,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
/// This enumeration identifies the visibility of messages in advanced queuing.
pub enum ODPIVisibility {
    /// The message is not part of the current transaction but constitutes a transaction of its own.
    Immediate = 1,
    /// The message is part of the current transaction. This is the default value.
    OnCommit = 2,
}
