//! ODPI-C opaque structs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// This structure represents connections to the database and is available by handle to a calling
/// application or driver.
pub struct ODPIConn([u8; 0]);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// This structure represents the context in which all activity in the library takes place.
pub struct dpiContext([u8; 0]);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
/// This structure represents session pools and is available by handle to a calling application or
/// driver.
pub struct ODPIPool([u8; 0]);
