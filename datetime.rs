// Copyright 2012 Derek A. Rhodes. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std;
import libc::*;
import str::unsafe;
import result::{result, ok, err};

import gdatetime::c;
import timezone::timezone;

// -------------------------------------------------------
// Datetime

fn new_now(tz: *gdatetime::GTimeZone) -> datetime {
    datetime(c::g_date_time_new_now(tz))
}

fn new_now_local() -> datetime {
    datetime(c::g_date_time_new_now_local())
}

fn new_now_utc() -> datetime {
    datetime(c::g_date_time_new_now_utc())
}

fn new_from_unix_local(t: i64) -> datetime {
    datetime(c::g_date_time_new_from_unix_local(t))
}

fn new_from_unix_utc(t: i64) -> datetime {
    datetime(c::g_date_time_new_from_unix_utc(t))
}

fn new_from_timeval_local(tv: *gdatetime::GTimeVal) -> datetime {
    datetime(c::g_date_time_new_from_timeval_local(tv))
}

fn new_from_timeval_utc(tv: *gdatetime::GTimeVal) -> datetime {
    datetime(c::g_date_time_new_from_timeval_utc(tv))
}

fn new_datetime(tz: timezone, year: int, month: int, day: int, 
                hour: int, minute: int, seconds: f32) -> datetime{
    datetime(c::g_date_time_new(
        tz.cref, 
        year as c_int, 
        month as c_int,
        day as c_int,
        hour as c_int, 
        minute as c_int,
        seconds as c_double))
}

fn new_datetime_local(year: int, month: int, day: int, 
                hour: int, minute: int, seconds: f32) -> datetime{
    datetime(c::g_date_time_new_local(
        year as c_int, 
        month as c_int,
        day as c_int,
        hour as c_int, 
        minute as c_int,
        seconds as c_double))
}

fn new_datetime_utc(year: int, month: int, day: int, 
                hour: int, minute: int, seconds: f32) -> datetime{
    datetime(c::g_date_time_new_utc(
        year as c_int, 
        month as c_int,
        day as c_int,
        hour as c_int, 
        minute as c_int,
        seconds as c_double))
}

class datetime {
    let cref: *gdatetime::GDateTime;
    
    new(gdt: *gdatetime::GDateTime) {
        self.cref = gdt;
    }

    drop {
        c::g_date_time_unref(self.cref);        
    }

    /// Clone this datetime
    fn clone() -> datetime {
        c::g_date_time_ref(self.cref);        
        datetime(self.cref)
    }

    /// Creates a copy of datetime adding the specified number of seconds.
    fn add_seconds(seconds: f64) -> datetime {
        let dt = c::g_date_time_add_seconds(self.cref, seconds as c_double);
        datetime(dt)            
    }
    
    /// Creates a copy of datetime adding the specified number of minutes.
    fn add_minutes(minutes: int) -> datetime {
        let dt = c::g_date_time_add_minutes(self.cref, minutes as c_int);
        datetime(dt)
    }

    /// Creates a copy of datetime adding the specified number of days.
    fn add_days(days: int) -> datetime {
        let dt = c::g_date_time_add_days(self.cref, days as c_int);
        datetime(dt)            
    }

    /// Creates a copy of datetime adding the specified number of months.
    fn add_months(months: int) -> datetime {
        let dt = c::g_date_time_add_months(self.cref, months as c_int);
        datetime(dt)            
    }


    /// Creates a copy of datetime adding the specified number of weeks.
    fn add_weeks(weeks: int) -> datetime {
        let dt = c::g_date_time_add_weeks(self.cref, weeks as c_int);
        datetime(dt)            
    }

    /// Creates a copy of datetime adding the specified number of years.
    fn add_years(years: int) -> datetime {
        let dt = c::g_date_time_add_years(self.cref, years as c_int);
        datetime(dt)            
    }

    // fn add (timespan: GTimeSpan ){
    // }
    // fn add_full (years: c_int,
    //              months: c_int, days: c_int, hours: c_int,
    //              minutes: c_int, seconds: c_double) -> *GDateTime;

    //fn int g_date_time_compare (gconstpointer dt1, // gconstpointer dt2);
    // GTimeSpan g_date_time_difference (GDateTime *end, // GDateTime *begin);

    fn hash() -> uint { 
        c::g_date_time_hash(self.cref) as uint
    }

    fn equal(other: datetime) -> bool {
        c::g_date_time_equal(self.cref, other.cref)
    }

    fn get_year() -> int {
        c::g_date_time_get_year(self.cref) as int
    }
    fn get_month() -> int {
        c::g_date_time_get_month(self.cref) as int
    }
    fn get_day_of_month() -> int {
        c::g_date_time_get_day_of_month(self.cref) as int
    }
    fn get_week_numbering_year() -> int {
        c::g_date_time_get_week_numbering_year(self.cref) as int
    }
    fn get_week_of_year() -> int {
        c::g_date_time_get_week_of_year(self.cref) as int
    }
    fn get_day_of_week() -> int {
        c::g_date_time_get_day_of_week(self.cref) as int
    }
    fn get_day_of_year() -> int {
        c::g_date_time_get_day_of_year(self.cref) as int
    }
    fn get_hour() -> int {
        c::g_date_time_get_hour(self.cref) as int
    }
    fn get_minute() -> int {
        c::g_date_time_get_minute(self.cref) as int
    }
    fn get_second() -> int {
        c::g_date_time_get_second(self.cref) as int
    }
    fn get_microsecond() -> int {
        c::g_date_time_get_microsecond(self.cref) as int
    }
    fn get_seconds () -> int {
        c::g_date_time_get_seconds(self.cref) as int
    }
    
    fn g_date_time_to_unix() -> i64 {
        c::g_date_time_to_unix(self.cref) as i64
    }

    // fn g_date_time_to_timeval (datetime: *GDateTime, tv: *GTimeVal) -> bool;

    fn get_utc_offset () -> i64 {
        c::g_date_time_get_utc_offset (self.cref) as i64
    }
    
    unsafe fn get_timezone_abbreviation () -> ~str {
        let cstr = c::g_date_time_get_timezone_abbreviation (self.cref);
        unsafe::from_c_str(cstr)
    }
    fn is_daylight_savings() -> bool {
        c::g_date_time_is_daylight_savings(self.cref) as bool
    }

    fn to_timezone(tz: timezone) -> datetime {
        datetime(c::g_date_time_to_timezone(self.cref, tz.cref))
    }
    fn to_local() -> datetime {
        datetime(c::g_date_time_to_local(self.cref))
    }
    fn to_utc() -> datetime {
        datetime(c::g_date_time_to_utc (self.cref))
    }

    fn format(format: ~str) -> result<~str, ~str> {
        gdatetime::format_datetime(self.cref, format)
    }
}

