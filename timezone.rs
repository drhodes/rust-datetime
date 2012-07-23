use std;
import libc::*;
import str::unsafe;
import result::{result, ok, err};

import gdatetime::c;
// -------------------------------------------------------
// TimeZone

enum timetype {
    Standard = 0,
    Daylight,
    Universal,
}

// Constructors
// Create a new instance of timezone can either be an RFC3339/ISO 8601 time offset 
// something that would pass as a valid value for the TZ environment variable (including NULL)
fn timezone_new(id: ~str) -> result<timezone, ~str> {
    let tz = str::as_c_str(id, c::g_time_zone_new);
    if tz == ptr::null() {
        ok(timezone(tz))
    } else {
        err(~"Received invalid timezone string: " + id)
    }
}

// Creates a datetime corresponding to local time. The local 
// timee zone may change between invocations to this function; 
// for example, if the system administrator changes it.
fn timezone_new_local() -> timezone {
    timezone(c::g_time_zone_new_local())
}

// Creates a timezone corresponding to UTC. This is equivalent 
// calling timezone_new() with a value like `Z`, `UTC`, `+00`, etc."]
fn timezone_new_utc() -> timezone {
    timezone(c::g_time_zone_new_utc())
}

// timezone is a structure that represents a time zone, at no particular
// point in time. It owns a pointer to a GTimeZone that is refcounted and immutable.
// full c docs @ http://developer.gnome.org/glib/2.31/glib-GTimeZone.html
class timezone {
    let cref: *gdatetime::GTimeZone;
    
    new(tz: *gdatetime::GTimeZone) {
        self.cref = tz;
    }
    drop {
        c::g_time_zone_unref(self.cref);
    }
    
    // Finds an the interval within tz that corresponds to the 
    // given time. The meaning of time depends on timetype 
    // If timetype is Universal then this function will 
    // always succeed (since universal time is monotonic and continuous)
    fn find_interval(tt: timetype, time: i64) -> int {
        c::g_time_zone_find_interval (self.cref, tt as c_uint, time) as int
    }

    // Finds an interval within tz that corresponds to the given time, 
    // possibly adjusting time if required to fit into an interval. 
    // The meaning of time depends on type."]
    fn adjust_time (tt: timetype, time: *i64) -> int {
        c::g_time_zone_adjust_time(self.cref, tt as c_uint, time) as int
    }

    // Determines the time zone abbreviation to be used during a particular
    //  interval of time in the time zone tz. For example, in Toronto this is
    //  currently `EST` during the winter months and `EDT` during the summer
    //  months when daylight savings time is in effect."]
    unsafe fn get_abbreviation (interval: int) -> ~str {
        let carr = c::g_time_zone_get_abbreviation(self.cref, interval as c_int);
        unsafe::from_c_str(carr)
    }
    
    // "Determines the offset to UTC in effect during a particular interval
    //  of time in the time zone tz. The offset is the number of seconds that
    //  you add to UTC time to arrive at local time for tz (ie: negative
    //  numbers for time zones west of GMT, positive numbers for east)."]
    fn get_offset (interval: int) -> i32 {
        c::g_time_zone_get_offset(self.cref, interval as c_int) as i32
    }

    // "Determines if daylight savings time is in effect during a particular
    //  interval of time in the time zone tz."]
    fn is_dst (interval: int) -> bool {
        c::g_time_zone_is_dst (self.cref, interval as c_int) as bool
    }
}
