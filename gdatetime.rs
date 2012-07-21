// Copyright 2012 Derek A. Rhodes. All rights reserved.

// This library is free software; you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2 of the
// licence, or (at your option) any later version.

// This library is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307,
// USA.

import libc::*;
import str::unsafe;
import result::{result, ok, err};

type GTimeSpan = i64;
const G_TIME_SPAN_DAY: i64 = 86400000000;
const G_TIME_SPAN_HOUR: i64 = 3600000000;
const G_TIME_SPAN_MINUTE: i64 = 60000000;
const G_TIME_SPAN_SECOND: i64 = 1000000;
const G_TIME_SPAN_MILLISECOND: i64 = 1000;

type GDateWeekday = u8;
type GDateDay = u8;
type GDateMonth = u8;
type GDateYear = u16;
type GTime = i32;

enum GDateTime{}
enum GDate{}
enum GTimeZone{}
//enum GTimeVal{}
type GTimeType = c_uint;

#[link_name = "glib-2.0"]
extern mod c {
    // timezone
    fn g_time_zone_unref (tz: *GTimeZone);
    fn g_time_zone_ref (tz: *GTimeZone) -> *GTimeZone;
    fn g_time_zone_new (identifier: *c_char) -> *GTimeZone;
    fn g_time_zone_new_local () -> *GTimeZone;
    fn g_time_zone_new_utc () -> *GTimeZone;
    fn g_time_zone_find_interval (tz: *GTimeZone, type_: GTimeType, time_: i64) -> c_int;
    fn g_time_zone_adjust_time (tz: *GTimeZone, type_: GTimeType, time_: *i64) -> c_int;
    fn g_time_zone_get_abbreviation (tz: *GTimeZone, interval: c_int) -> *c_char;
    fn g_time_zone_get_offset (tz: *GTimeZone, interval: c_int) -> i32;
    fn g_time_zone_is_dst (tz: *GTimeZone, interval: c_int) -> bool;

    // datetime
    fn g_date_time_unref(datetime: *GDateTime);
    fn g_date_time_ref (datetime: *GDateTime) -> *GDateTime;

    fn g_date_time_new_now(tz: *GTimeZone) -> *GDateTime;
    fn g_date_time_new_now_local() -> *GDateTime;
    fn g_date_time_new_now_utc() -> *GDateTime;

    fn g_date_time_new_from_unix_local (t: i64) -> *GDateTime;
    fn g_date_time_new_from_unix_utc (t: i64) -> *GDateTime;

    //fn g_date_time_new_from_timeval_local ( tv: *GTimeVal) -> *GDateTime;
    //fn g_date_time_new_from_timeval_utc ( tv: *GTimeVal) -> *GDateTime;
    
    fn g_date_time_new (tz: *GTimeZone, year: c_int, month: c_int, day: c_int, 
                        hour: c_int, minute: c_int, seconds: c_double) -> *GDateTime;
    fn g_date_time_new_local (year: c_int, month: c_int, day: c_int, 
                              hour: c_int, minute: c_int, seconds: c_double) -> *GDateTime;
    fn g_date_time_new_utc (year: c_int, month: c_int, day: c_int, 
                            hour: c_int, minute: c_int, seconds: c_double) -> *GDateTime;

    fn g_date_time_add_minutes(datetime: *GDateTime, minutes: c_int) -> *GDateTime;
    fn g_date_time_add_months(datetime: *GDateTime, months: c_int) -> *GDateTime;
    fn g_date_time_add_seconds(datetime: *GDateTime, seconds: c_double) -> *GDateTime;
    fn g_date_time_add_weeks(datetime: *GDateTime, weeks: c_int) -> *GDateTime;
    fn g_date_time_add_years(datetime: *GDateTime, years: c_int) -> *GDateTime;
    
    fn g_date_time_add_days (datetime: *GDateTime, days: c_int) -> *GDateTime;
    fn g_date_time_add_hours (datetime: *GDateTime, hours: c_int) -> *GDateTime;
    //fn g_date_time_add (datetime: *GDateTime, timespan: GTimeSpan ) -> *GDateTime;
    fn g_date_time_add_full (datetime: *GDateTime, years: c_int, 
                             months: c_int, days: c_int, hours: c_int,
                             minutes: c_int, seconds: c_double) -> *GDateTime;
    
    //: c_int g_date_time_compare (gconstpointer dt1, // gconstpointer dt2);
    // GTimeSpan g_date_time_difference (GDateTime *end, // GDateTime *begin);
    fn g_date_time_hash (dt: *GDateTime) -> c_uint;
    // void g_date_time_get_ymd (GDateTime *datetime, //: c_int *year, //: c_int *month, //: c_int *day);

    fn g_date_time_equal(dt1: *GDateTime, dt2: *GDateTime) -> bool;

    
    fn g_date_time_get_year (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_month (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_day_of_month (datetime: *GDateTime) -> c_int;
    
    fn g_date_time_get_week_numbering_year (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_week_of_year (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_day_of_week (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_day_of_year (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_hour (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_minute (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_second (datetime: *GDateTime) -> c_int;
    fn g_date_time_get_microsecond (datetime: *GDateTime) -> c_int;

    fn g_date_time_get_seconds (datetime: *GDateTime) -> c_double;

    fn g_date_time_to_unix (datetime: *GDateTime) -> i64;
    //fn g_date_time_to_timeval (datetime: *GDateTime, tv: *GTimeVal) -> bool;

    fn g_date_time_get_utc_offset (datetime: *GDateTime) -> GTimeSpan;
    fn g_date_time_get_timezone_abbreviation (datetime: *GDateTime) -> *c_char;
    fn g_date_time_is_daylight_savings (datetime: *GDateTime) -> bool;
    
    fn g_date_time_to_timezone (datetime: *GDateTime, tz: *GTimeZone) -> *GDateTime;
    fn g_date_time_to_local (datetime: *GDateTime) -> *GDateTime;
    fn g_date_time_to_utc (datetime: *GDateTime) -> *GDateTime;
    
    fn g_date_time_format (datetime: *GDateTime, format: *c_char) -> *c_char;
}

fn format_datetime(datetime: *GDateTime, format: ~str) -> result<~str, ~str> {
    let fmtd = str::as_c_str(format, {|x| c::g_date_time_format(datetime, x)});
    unsafe {
        if fmtd == ptr::null() {
            err(~"bad format string: " + format)
        } else {
            ok(unsafe::from_c_str(fmtd))
        }
    }    
}
