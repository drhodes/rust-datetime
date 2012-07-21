use std;
import datetime::*;
import timezone::*;

#[test]
fn test_timezone() {
    let t = timezone_new_local();
    log(error, t.is_dst(0)); // is this right?
    log(error, t.get_abbreviation(1));
}