use std;
import libc::*;
import str::unsafe;

import gdatetime::c;
//import timezone::*;

// -------------------------------------------------------
// Datetime

fn new_now(tz: *gdatetime::GTimeZone) -> datetime {
    datetime(c::g_date_time_new_now(tz))
}

//    fn g_date_time_new_now_local() -> *GDateTime;
//    fn g_date_time_new_now_utc() -> *GDateTime;
//    fn g_date_time_new_from_unix_local (t: i64) -> *GDateTime;
//    fn g_date_time_new_from_unix_utc (t: i64) -> *GDateTime;
//  fn g_date_time_new_from_timeval_local ( tv: *GTimeVal) -> *GDateTime;
//    fn g_date_time_new_from_timeval_utc ( tv: *GTimeVal) -> *GDateTime;
// fn g_date_time_new (tz: *GTimeZone, year: c_int, month: c_int, day: c_int,
//                     hour: c_int, minute: c_int, seconds: c_double) -> *GDateTime;
// fn g_date_time_new_local (year: c_int, month: c_int, day: c_int,
//                           hour: c_int, minute: c_int, seconds: c_double) -> *GDateTime;
// fn g_date_time_new_utc (year: c_int, month: c_int, day: c_int,
//                         hour: c_int, minute: c_int, seconds: c_double) -> *GDateTime;

// functional implementation, fresh copies abound.
class datetime {
    let gdt: *gdatetime::GDateTime;
    
    new(gdt: *gdatetime::GDateTime) {
        self.gdt = gdt;
        c::g_date_time_ref(self.gdt);
    }

    drop {
        c::g_date_time_unref(self.gdt);
    }

    // #[doc = "Clone a datetime and its underlying c structures"]
    // fn clone() -> datetime {
    //     // TODO: find a faster way to do this


    // }

    // #[doc = "Creates a copy of datetime adding the specified number of minutes."]
    // fn add_minutes(minutes: int) -> datetime {
    //     let dt = g_date_time_add_minutes(self, minutes as c_int);

    // }

    // fn add_months(months: c_int){
    // }
    // fn add_seconds(seconds: c_int){
    // }
    // fn add_weeks(weeks: c_int){
    // }
    // fn add_years(years: c_int){
    // }

    // fn add_days (days: c_int){
    // }
    // fn add_hours (hours: c_int){
    // }
    // fn add (timespan: GTimeSpan ){
    // }
    // fn add_full (years: c_int,
    //              months: c_int, days: c_int, hours: c_int,
    //              minutes: c_int, seconds: c_double) -> *GDateTime;

    //: c_int g_date_time_compare (gconstpointer dt1, // gconstpointer dt2);
    // GTimeSpan g_date_time_difference (GDateTime *end, // GDateTime *begin);
    // guint g_date_time_hash (gconstpointer datetime);
    // gboolean g_date_time_equal (gconstpointer dt1, // gconstpointer dt2);
    // void g_date_time_get_ymd (GDateTime *datetime, //: c_int *year, //: c_int *month, //: c_int *day);

    // fn g_date_time_get_year (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_month (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_day_of_month (datetime: *GDateTime) -> c_int;

    // fn g_date_time_get_week_numbering_year (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_week_of_year (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_day_of_week (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_day_of_year (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_hour (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_minute (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_second (datetime: *GDateTime) -> c_int;
    // fn g_date_time_get_microsecond (datetime: *GDateTime) -> c_int;

    // fn g_date_time_get_seconds (datetime: *GDateTime) -> c_double;

    // fn g_date_time_to_unix (datetime: *GDateTime) -> i64;
    // fn g_date_time_to_timeval (datetime: *GDateTime, tv: *GTimeVal) -> bool;

    // fn g_date_time_get_utc_offset (datetime: *GDateTime) -> GTimeSpan;
    // fn g_date_time_get_timezone_abbreviation (datetime: *GDateTime) -> *c_char;
    // fn g_date_time_is_daylight_savings (datetime: *GDateTime) -> bool;

    // fn g_date_time_to_timezone (datetime: *GDateTime, tz: *GTimeZone) -> *GDateTime;
    // fn g_date_time_to_local (datetime: *GDateTime) -> *GDateTime;
    // fn g_date_time_to_utc (datetime: *GDateTime) -> *GDateTime;

    // fn g_date_time_format (datetime: *GDateTime, format: *c_char) -> *c_char;
}

// impl of to_str::to_str for Datetime {
//     fn to_str() -> ~str {
//         if self.
//     }
// }