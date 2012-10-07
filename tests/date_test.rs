// use std;
use gdatetime::c;
use gdatetime::*;

#[test]
pub fn sane_formatting() {
    let here = c::g_time_zone_new_local();    
    let t = c::g_date_time_new(here, 1984, 11, 5, 0, 0, 0.0 as c_double);

    assert gdatetime::format_datetime(t, ~"%T") == Ok(~"00:00:00");
    assert gdatetime::format_datetime(t, ~"%&&") == Err(~"bad format string: %&&");
    assert gdatetime::format_datetime(t, ~"%F") == Ok(~"1984-11-05");

    // %a: the abbreviated weekday name according to the current locale
    //log(error, format_datetime(t, ~"%a"));
    assert format_datetime(t, ~"%a") == Ok(~"Mon");
    // %A: the full weekday name according to the current locale
    //log(error, format_datetime(t, ~"%A"));
    assert format_datetime(t, ~"%A") == Ok(~"Monday");
    // // %b: the abbreviated month name according to the current locale
    // //log(error, format_datetime(t, ~"%b"));
    assert format_datetime(t, ~"%b") == Ok(~"Nov");
    // // %B: the full month name according to the current locale
    // //log(error, format_datetime(t, ~"%B"));
    assert format_datetime(t, ~"%B") == Ok(~"November");
    // // %c: the preferred date and time representation for the current locale
    // //log(error, format_datetime(t, ~"%c"));
    assert format_datetime(t, ~"%c") == Ok(~"Mon Nov  5 00:00:00 1984");
    // // %C: The century number (year/100) as a 2-digit integer (00-99)
    // //log(error, format_datetime(t, ~"%C"));
    assert format_datetime(t, ~"%C") == Ok(~"19");
    // // %d: the day of the month as a decimal number (range 01 to 31)
    // //log(error, format_datetime(t, ~"%d"));
    assert format_datetime(t, ~"%d") == Ok(~"05");
    // // %e: the day of the month as a decimal number (range 1 to 31)
    // //log(error, format_datetime(t, ~"%e"));
    assert format_datetime(t, ~"%e") == Ok(~" 5");
    // // %F: equivalent to %Y-%m-%d (the ISO 8601 date format)
    // //log(error, format_datetime(t, ~"%F"));
    assert format_datetime(t, ~"%F") == Ok(~"1984-11-05");
    // // %g: the last two digits of the ISO 8601 week-based year as a
    // // decimal number (00-99). This works well with %V and %u.
    // //log(error, format_datetime(t, ~"%g"));
    assert format_datetime(t, ~"%g") == Ok(~"84");
    // // %G: the ISO 8601 week-based year as a decimal number. This works well with %V and %u.
    // //log(error, format_datetime(t, ~"%G"));
    assert format_datetime(t, ~"%G") == Ok(~"1984");
    // // %h: equivalent to %b
    // //log(error, format_datetime(t, ~"%h"));
    assert format_datetime(t, ~"%h") == Ok(~"Nov");
    // // %H: the hour as a decimal number using a 24-hour clock (range 00 to 23)
    // //log(error, format_datetime(t, ~"%H"));
    assert format_datetime(t, ~"%H") == Ok(~"00");
    // // %I: the hour as a decimal number using a 12-hour clock (range 01 to 12)
    // //log(error, format_datetime(t, ~"%I"));
    assert format_datetime(t, ~"%I") == Ok(~"12");
    // // %j: the day of the year as a decimal number (range 001 to 366)
    // //log(error, format_datetime(t, ~"%j"));
    assert format_datetime(t, ~"%j") == Ok(~"310");
    // // %k: the hour (24-hour clock) as a decimal number (range 0 to 23); single digits are preceded by a blank
    // //log(error, format_datetime(t, ~"%k"));
    assert format_datetime(t, ~"%k") == Ok(~" 0");
    // // %l: the hour (12-hour clock) as a decimal number (range 1 to 12); single digits are preceded by a blank
    // //log(error, format_datetime(t, ~"%l"));
    assert format_datetime(t, ~"%l") == Ok(~"12");
    // // %m: the month as a decimal number (range 01 to 12)
    // //log(error, format_datetime(t, ~"%m"));
    assert format_datetime(t, ~"%m") == Ok(~"11");
    // // %M: the minute as a decimal number (range 00 to 59)
    // //log(error, format_datetime(t, ~"%M"));
    assert format_datetime(t, ~"%M") == Ok(~"00");
    // // %p: either "AM" or "PM" according to the given time value, or the 
    // // corresponding strings for the current locale. Noon is treated as "PM" and midnight as "AM".
    // //log(error, format_datetime(t, ~"%p"));
    assert format_datetime(t, ~"%p") == Ok(~"AM");
    // // %P: like %p but lowercase: "am" or "pm" or a corresponding string for the current locale
    // //log(error, format_datetime(t, ~"%P"));
    assert format_datetime(t, ~"%P") == Ok(~"am");
    // // %r: the time in a.m. or p.m. notation
    // //log(error, format_datetime(t, ~"%r"));
    assert format_datetime(t, ~"%r") == Ok(~"12:00:00 AM");
    // // %R: the time in 24-hour notation (%H:%M)
    // //log(error, format_datetime(t, ~"%R"));
    assert format_datetime(t,  ~"%R") == Ok(~"00:00");
    // // %s: the number of seconds since the Epoch, that is, since 1970-01-01 00:00:00 UTC
    // //log(error, format_datetime(t, ~"%s"));
    assert format_datetime(t, ~"%s") == Ok(~"468478800");
    // // %S: the second as a decimal number (range 00 to 60)
    // //log(error, format_datetime(t, ~"%S"));
    assert format_datetime(t, ~"%S") == Ok(~"00");
    // // %t: a tab character
    // //log(error, format_datetime(t, ~"%t"));
    assert format_datetime(t, ~"%t") == Ok(~"\t");
    // // %T: the time in 24-hour notation with seconds (%H:%M:%S)
    // //log(error, format_datetime(t, ~"%T"));
    assert format_datetime(t, ~"%T") == Ok(~"00:00:00");
    // // %u: the ISO 8601 standard day of the week as a decimal, 
    // // range 1 to 7, Monday being 1. This works well with %G and %V.
    // //log(error, format_datetime(t, ~"%u"));
    assert format_datetime(t, ~"%u") == Ok(~"1");
    // // %V: the ISO 8601 standard week number of the current year as a decimal number
    // //log(error, format_datetime(t, ~"%V"));
    assert format_datetime(t, ~"%V") == Ok(~"45");
    // // range 01 to 53, where week 1 is the first week that has at least 4 days in 
    // //  the new year. See g_date_time_get_week_of_year(). This works well with %G and %u.
    // // %w: the day of the week as a decimal, range 0 to 6, Sunday being 0. 
    // // This is not the ISO 8601 standard format -- use %u instead.
    // //log(error, format_datetime(t, ~"%w"));
    assert format_datetime(t, ~"%w") == Ok(~"1");
    // // %x: the preferred date representation for the current locale without the time
    // //log(error, format_datetime(t, ~"%x"));
    assert format_datetime(t, ~"%x") == Ok(~"11/05/84");
    // // %X: the preferred time representation for the current locale without the date
    // //log(error, format_datetime(t, ~"%X"));
    assert format_datetime(t, ~"%X") == Ok(~"00:00:00");
    // // %y: the year as a decimal number without the century
    // //log(error, format_datetime(t, ~"%y"));
    assert format_datetime(t, ~"%y") == Ok(~"84");
    // // %Y: the year as a decimal number including the century
    // //log(error, format_datetime(t, ~"%Y"));
    assert format_datetime(t, ~"%Y") == Ok(~"1984");
    // // %z: the time-zone as hour offset from UTC
    // //log(error, format_datetime(t, ~"%z"));
    assert format_datetime(t, ~"%z") == Ok(~"-0500");
    // // %Z: the time zone or name or abbreviation
    // //log(error, format_datetime(t, ~"%Z"));
    assert format_datetime(t, ~"%Z") == Ok(~"EST");
}

