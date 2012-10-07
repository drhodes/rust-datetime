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

// Function comments are copyrighted by 
// Copyright © 2010 Codethink Limited
// Author: Ryan Lortie <desrt@desrt.ca>
// The comments have been altered to suit Rust's type system.

use libc::*;
use str::raw;
use result::{Result, Ok, Err};

// use gdatetime::c;
// use gdatetime;

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
fn TimezoneNew(id: ~str) -> Result<Timezone, ~str> {
    let tz = str::as_c_str(id, c::g_time_zone_new);
    if tz == ptr::null() {
        Ok(Timezone(tz))
    } else {
        Err(~"Received invalid timezone string: " + id)
    }
}

// Creates a datetime corresponding to local time. The local 
// timee zone may change between invocations to this function; 
// for example, if the system administrator changes it.
fn TimezoneNewLocal() -> Timezone {
    Timezone(c::g_time_zone_new_local())
}

// Creates a timezone corresponding to UTC. This is equivalent 
// calling timezone_new() with a value like `Z`, `UTC`, `+00`, etc."]
fn TimezoneNewUtc() -> Timezone {
    Timezone(c::g_time_zone_new_utc())
}

// timezone is a structure that represents a time zone, at no particular
// point in time. It owns a pointer to a GTimeZone that is refcounted and immutable.
// full c docs @ http://developer.gnome.org/glib/2.31/glib-GTimeZone.html
struct Timezone {
    cref: *gdatetime::GTimeZone,

    drop {
        c::g_time_zone_unref(self.cref);
    }
}
    
fn Timezone(tz: *gdatetime::GTimeZone) -> Timezone {
    Timezone(tz)
}

impl Timezone {
    /// Clone this timezone
    /// Replace this with copy constructor later.
    fn clone() -> Timezone {
        c::g_time_zone_ref(self.cref);        
        Timezone(self.cref)
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
        raw::from_c_str(carr)
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

