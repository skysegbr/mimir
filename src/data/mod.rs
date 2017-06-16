// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! All of these functions are used for getting and setting the various members of the dpiData
//! structure. The members of the structure can be manipulated directly but some languages
//! (such as Go) do not have the ability to manipulate structures containing unions or the ability
//! to process macros. For this reason, none of these functions perform any error checking. They are
//! assumed to be replacements for direct manipulation of the various members of the structure.
use chrono::{DateTime, Duration, TimeZone, UTC};
use odpi::opaque;
use odpi::structs::{ODPIData, ODPIDataValueUnion};
use util::ODPIStr;

/// This structure is used for holding Oracle year to month interval data information.
#[derive(Default, Getters, Setters)]
pub struct YearsMonths {
    /// The years in an Oracle YEARS TO MONTHS interval.
    #[get = "pub"]
    #[set = "pub"]
    years: i32,
    /// The months in an Oracle YEARS TO MONTHS interval.
    #[get = "pub"]
    #[set = "pub"]
    months: i32,
}

/// This structure is used for passing data to and from the database for variables and for
/// manipulating object attributes and collection values.
#[derive(Debug)]
pub struct Data {
    /// The ODPI-C data pointer.
    inner: *mut ODPIData,
}

impl Data {
    /// Create a new `Data` struct;
    #[doc(hidden)]
    pub fn new(is_null: bool, val: ODPIDataValueUnion) -> Data {
        let mut odpi_data = ODPIData {
            is_null: if is_null { 1 } else { 0 },
            value: val,
        };
        Data { inner: &mut odpi_data }
    }

    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIData {
        self.inner
    }

    /// Get the value as a boolean when the native type is DPI_NATIVE_TYPE_BOOLEAN.
    pub fn as_boolean(&self) -> bool {
        unsafe { (*self.inner).value.as_boolean == 1 }
    }

    /// Get the value as an `i64` when the native type is DPI_NATIVE_TYPE_INT64.
    pub fn as_int64(&self) -> i64 {
        unsafe { (*self.inner).value.as_int_64 }
    }

    /// Get the value as a `u64` when the native type is DPI_NATIVE_TYPE_UINT64.
    pub fn as_uint64(&self) -> u64 {
        unsafe { (*self.inner).value.as_uint_64 }
    }

    /// Get the value as a `f32` when the native type is DPI_NATIVE_TYPE_FLOAT.
    pub fn as_float(&self) -> f32 {
        unsafe { (*self.inner).value.as_float }
    }

    /// Get the value as a `f64` when the native type is DPI_NATIVE_TYPE_DOUBLE.
    pub fn as_double(&self) -> f64 {
        unsafe { (*self.inner).value.as_double }
    }

    /// Returns the value of the data when the native type is DPI_NATIVE_TYPE_OBJECT.
    pub fn as_object(&self) -> *mut opaque::ODPIObject {
        unsafe { (*self.inner).value.as_object }
    }

    /// Get the value as a `String` when the native type is DPI_NATIVE_TYPE_BYTES.
    pub fn as_string(&self) -> String {
        unsafe {
            let odpi_bytes = (*self.inner).value.as_bytes;
            let odpi_s = ODPIStr::new(odpi_bytes.ptr, odpi_bytes.length);
            odpi_s.into()
        }
    }

    /// Get the value as a `UTC` when the native type is DPI_NATIVE_TYPE_TIMESTAMP.
    pub fn as_utc(&self) -> DateTime<UTC> {
        let odpi_ts = unsafe { (*self.inner).value.as_timestamp };
        let y = odpi_ts.year as i32;
        let m = odpi_ts.month as u32;
        let d = odpi_ts.day as u32;
        let h = odpi_ts.hour as u32;
        let mi = odpi_ts.minute as u32;
        let s = odpi_ts.second as u32;
        UTC.ymd(y, m, d).and_hms_nano(h, mi, s, odpi_ts.fsecond)
    }

    /// Get the value as a `Duration` when the native type is DPI_NATIVE_TYPE_INTERVAL_DS.
    pub fn as_duration(&self) -> Duration {
        let odpi_int_ds = unsafe { (*self.inner).value.as_interval_ds };
        let mut dur = Duration::days(odpi_int_ds.days as i64);
        dur = dur + Duration::hours(odpi_int_ds.hours as i64);
        dur = dur + Duration::minutes(odpi_int_ds.minutes as i64);
        dur = dur + Duration::seconds(odpi_int_ds.seconds as i64);
        dur = dur + Duration::nanoseconds(odpi_int_ds.fseconds as i64);
        dur
    }

    /// Get the value as a `YearsMonths` when the native type is DPI_NATIVE_TYPE_INTERVAL_YM.
    pub fn as_years_months(&self) -> YearsMonths {
        let odpi_int_ym = unsafe { (*self.inner).value.as_interval_ym };
        let mut ym: YearsMonths = Default::default();
        ym.set_years(odpi_int_ym.years);
        ym.set_months(odpi_int_ym.months);
        ym
    }
}

impl From<*mut ODPIData> for Data {
    fn from(inner: *mut ODPIData) -> Data {
        Data { inner: inner }
    }
}
