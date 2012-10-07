// Copyright 2012 Derek A. Rhodes. All rights reserved.
//
// This is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
//
// Function comments are copyrighted by
// Copyright (C) 2009-2010 Christian Hergert <chris@dronelabs.com>
// Altered to describe the calling convention of rust.

use libc::*;
use str::raw;
use result::{Result, Ok, Err};
use to_str;

use gdatetime;
use gdatetime::c;
use timezone::Timezone;

// The timespan methods aren't included yet.  if anyone needs them
// then please make an issue.  

// -------------------------------------------------------
// Datetime

/// Creates a GDateTime corresponding to this exact instant in the
/// given time zone tz. The time is as accurate as the system allows
/// to a maximum accuracy of 1 microsecond.
fn new_now(tz: Timezone) -> datetime {
    return datetime(c::g_date_time_new_now(tz.cref));
}

/// Creates a GDateTime corresponding to this exact instant in the
/// local time zone.
fn new_now_local() -> datetime {
    return datetime(c::g_date_time_new_now_local());
}

/// Creates a GDateTime corresponding to this exact instant in UTC.
fn new_now_utc() -> datetime {
    return datetime(c::g_date_time_new_now_utc());
}

/// Creates a datetime corresponding to the given Unix time t in
/// the local time zone. Unix time is the number of seconds that have
/// elapsed since 1970-01-01 00:00:00 UTC, regardless of the local time offset.
/// This call can fail (returns result::err) if t represents a time outside
/// of the supported range of GDateTime.
fn new_from_unix_local(t: i64) -> Result<datetime, ~str> {
    let gdt = c::g_date_time_new_from_unix_local(t);
    if gdt == ptr::null() {
        let msg = ~"error in datetime::new_from_unix_local, \
                    time outside supported range: ";
        return Err(msg + i64::to_str(t, 10));
    } else {
        return Ok(datetime(gdt));
    }        
}

/// Creates a GDateTime corresponding to the given Unix time t in
/// UTC. Unix time is the number of seconds that have elapsed since 
/// 1970-01-01 00:00:00 UTC. This call can fail (returns result::err) if
/// t represents a time outside of the supported range of GDateTime.
fn new_from_unix_utc(t: i64) -> Result<datetime, ~str> {
    let gdt = c::g_date_time_new_from_unix_utc(t);
    //log(error, gdt.tv_sec);
    if gdt == ptr::null() {
        let msg = ~"error in datetime::new_from_unix_utc, \
                    time outside supported range: ";
        Err(msg + i64::to_str(t, 10))
    } else {
        Ok(datetime(gdt))
    }        
}

// Creates a GDateTime corresponding to the given GTimeVal tv in
// the local time zone. The time contained in a GTimeVal is always
// stored in the form of seconds elapsed since 1970-01-01 00:00:00 UTC,
// regardless of the local time offset. This call can fail
// (returning result::err) if tv represents a time outside of the supported
// range of GDateTime.
// fn new_from_timeval_local(tv: *gdatetime::GTimeVal) -> Result<datetime, ~str> {
//     let gdt = c::g_date_time_new_from_timeval_local(tv);
//     if gdt == ptr::null() {
//         let msg = ~"error in datetime::new_from_timeval_local, \
//                     time val outside supported range: ";
//         err(msg + "TODO: string representation of GTimeVal")
//     } else {
//         Ok(datetime(gdt))
//     }        
// }

// fn new_from_timeval_utc(tv: *gdatetime::GTimeVal) -> datetime {
//     datetime(c::g_date_time_new_from_timeval_utc(tv))
// }

/// Creates a new datetime corresponding to the given date and time 
/// in the time zone tz
fn new_datetime(tz: Timezone, year: int, month: int, day: int, 
                hour: int, minute: int, seconds: f32) -> datetime{
    return datetime(c::g_date_time_new(
        tz.cref, 
        year as c_int, 
        month as c_int,
        day as c_int,
        hour as c_int, 
        minute as c_int,
        seconds as c_double));
}

/// Creates a new datetime corresponding to the given
/// date and time in the local time zone.
fn new_datetime_local(year: int, month: int, day: int, 
                      hour: int, minute: int, seconds: f32) -> datetime {
    return datetime(c::g_date_time_new_local(year as c_int, 
                                             month as c_int,
                                             day as c_int,
                                             hour as c_int, 
                                             minute as c_int,
                                             seconds as c_double));
}

/// Creates a new datetime corresponding to the given date and time in UTC.
fn new_datetime_utc(year: int, month: int, day: int, 
                    hour: int, minute: int, seconds: f32) -> datetime{
    return datetime(c::g_date_time_new_utc(
        year as c_int, 
        month as c_int,
        day as c_int,
        hour as c_int, 
        minute as c_int,
        seconds as c_double));
}

struct datetime {
    cref: *gdatetime::GDateTime,

    drop {
        c::g_date_time_unref(self.cref);        
    }
}    

fn datetime(gdt: *gdatetime::GDateTime) -> datetime {
    return datetime{cref: gdt};
}

impl datetime { 
    /// Clone this datetime
    /// Replace this with copy constructor later.
    fn clone() -> datetime {        
        c::g_date_time_ref(self.cref);        
        return datetime(self.cref);
    }
    
    /// Creates a copy of datetime adding the specified number of seconds.
    fn add_seconds(seconds: f64) -> datetime {
        let dt = c::g_date_time_add_seconds(self.cref, seconds as c_double);
        return datetime(dt);
    }
    
    /// Creates a copy of datetime adding the specified number of minutes.
    fn add_minutes(minutes: int) -> datetime {
        let dt = c::g_date_time_add_minutes(self.cref, minutes as c_int);
        return datetime(dt);
    }
    
    /// Creates a copy of datetime adding the specified number of days.
    fn add_days(days: int) -> datetime {
        let dt = c::g_date_time_add_days(self.cref, days as c_int);
        return datetime(dt);
    }
    
    /// Creates a copy of datetime adding the specified number of months.
    fn add_months(months: int) -> datetime {
        let dt = c::g_date_time_add_months(self.cref, months as c_int);
        return datetime(dt);
    }

    /// Creates a copy of datetime adding the specified number of weeks.
    fn add_weeks(weeks: int) -> datetime {
        let dt = c::g_date_time_add_weeks(self.cref, weeks as c_int);
        return datetime(dt);
    }

    /// Creates a copy of datetime adding the specified number of years.
    fn add_years(years: int) -> datetime {
        let dt = c::g_date_time_add_years(self.cref, years as c_int);
        return datetime(dt);
    }
    
    // fn add (timespan: GTimeSpan ){
    // }  
    
    /// Create a copy of datetime while adding any or all of the following 
    /// units of time: years, months, days, hours, minutes, seconds.
    fn add_full(years: int, months: int, days: int, hours: int,
                minutes: int, seconds: f64) -> Result<datetime, ~str> {        
        let gdt = c::g_date_time_add_full(self.cref, years as c_int, months as c_int, 
                                          days as c_int, hours as c_int, 
                                          minutes as c_int, seconds as c_double);
        if gdt == ptr::null() {
            let msg = ~"error in datetime::add_full, \
                        found value outside of supported range: ";
            return Err(msg);
        } else {
            return Ok(datetime(gdt));
        }        
    }    
    
    //fn int g_date_time_compare (gconstpointer dt1, // gconstpointer dt2);
    // GTimeSpan g_date_time_difference (GDateTime *end, // GDateTime *begin);

    /// Hashes datetime into a uint
    fn hash() -> uint { 
        return c::g_date_time_hash(self.cref) as uint;
    }

    /// Test for equality
    fn equal(other: datetime) -> bool {
        return c::g_date_time_equal(self.cref, other.cref) as bool;
    }
    
    /// Retrieves the year represented by datetime in the Gregorian calendar.    
    fn get_year() -> int {
        return c::g_date_time_get_year(self.cref) as int;
    }
    
    /// Retrieves the month of the year represented by datetime in the Gregorian calendar.
    fn get_month() -> int {
        c::g_date_time_get_month(self.cref) as int 
    }

    /// Retrieves the day of the month represented by datetime in the gregorian calendar.
    fn get_day_of_month() -> int {
        c::g_date_time_get_day_of_month(self.cref) as int
    }
    
    /// Returns the ISO 8601 week-numbering year in which the week containing datetime falls.
    fn get_week_numbering_year() -> int {
        c::g_date_time_get_week_numbering_year(self.cref) as int
    }

    /// Returns the ISO 8601 week number for the week containing datetime
    /// The ISO 8601 week number is the same for every day of the week
    /// (from Monday through Sunday). That can produce some unusual
    /// results (described at http://goo.gl/uJJnD)
    fn get_week_of_year() -> int {
        c::g_date_time_get_week_of_year(self.cref) as int
    }

    /// Retrieves the ISO 8601 day of the week on which datetime falls
    /// (1 is Monday, 2 is Tuesday... 7 is Sunday).
    fn get_day_of_week() -> int {
        c::g_date_time_get_day_of_week(self.cref) as int 
    }

    /// Retrieves the day of the year represented by datetime in the Gregorian calendar.
    fn get_day_of_year() -> int {
        c::g_date_time_get_day_of_year(self.cref) as int
    }

    /// Retrieves the hour of the day represented by datetime
    fn get_hour() -> int {
        c::g_date_time_get_hour(self.cref) as int
    }

    /// Retrieves the minute of the hour represented by datetime
    fn get_minute() -> int {
        c::g_date_time_get_minute(self.cref) as int
    }

    /// Retrieves the second of the minute represented by datetime
    fn get_second() -> int {
        c::g_date_time_get_second(self.cref) as int
    }

    /// Retrieves the microsecond of the date represented by datetime
    fn get_microsecond() -> int {
        c::g_date_time_get_microsecond(self.cref) as int
    }

    /// Retrieves the number of seconds since the start of the last minute
    /// including the fractional part.
    fn get_seconds () -> f64 {
        c::g_date_time_get_seconds(self.cref) as f64
    }
    
    /// Gives the Unix time corresponding to datetime, rounding down
    /// to the nearest second.
    fn to_unix() -> i64 {
        c::g_date_time_to_unix(self.cref) as i64
    }

    // fn to_timeval() -> TimeVal {
    //     c::g_date_time_to_timeval(self.cref, datetime: *GDateTime, tv: *GTimeVal) -> bool;
    // }

    /// Determines the offset to UTC in effect at the time and in 
    /// the time zone of datetime.
    fn get_utc_offset () -> i64 {
        c::g_date_time_get_utc_offset (self.cref) as i64
    }   

    /// Determines the time zone abbreviation to be used at the time
    /// and in the time zone of datetime.
    unsafe fn get_timezone_abbreviation () -> ~str {
        let cstr = c::g_date_time_get_timezone_abbreviation(self.cref);
        raw::from_c_str(cstr)
    }

    /// Determines if daylight savings time is in effect at the time
    /// and in the time zone of datetime.
    fn is_daylight_savings() -> bool {
        c::g_date_time_is_daylight_savings(self.cref) as bool
    }

    /// Create a new GDateTime corresponding to the same instant in time
    /// as datetime, but in the time zone tz.
    fn to_timezone(tz: Timezone) -> datetime {
        datetime(c::g_date_time_to_timezone(self.cref, tz.cref))
    }

    /// Creates a new GDateTime corresponding to the same instant in
    /// time as datetime, but in the local time zone.
    fn to_local() -> datetime {
        datetime(c::g_date_time_to_local(self.cref))
    }

    /// Creates a new datetime corresponding to the same instant in
    /// time as datetime, but in UTC.
    fn to_utc() -> datetime {
        return datetime(c::g_date_time_to_utc (self.cref));
    }

    /// Creates a newly allocated string representing the requested format.
    fn format(format: ~str) -> Result<~str, ~str> {
        gdatetime::format_datetime(self.cref, format)
    }
}
