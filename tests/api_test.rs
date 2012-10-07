use datetime::*;
use timezone::*;

// check to see if your timezone is correct.
// #[test]
// fn test_timezone() {
//     let t = timezone_new_local();
//     log(error, t.is_dst(0)); // is this right?
//     log(error, t.get_abbreviation(1));
// }

#[test]
pub fn test_adders() {
    match new_from_unix_utc(0) {
        Err(s) => fail(s),
        Ok(t) => {
            assert t.format(~"%c") == Ok(~"Thu Jan  1 00:00:00 1970");
            assert t.equal(t.clone()) == true;
            assert t.equal(t.add_years(1)) == false;
            assert t.equal(t.add_seconds(0.001 as f64)) == false;
            assert t.add_seconds(86400 as f64).equal(t.add_days(1));
        }
    }
}

fn id(dt: datetime) -> datetime {
    dt.clone()
}

#[test]
pub fn test_clone_one() {
    let mut d1: datetime;
    let mut d2: datetime;

    match new_from_unix_utc(0) {
        Err(s) => fail(s),
        Ok(t) => {
            d1 = t.clone();
            d2 = d1.clone();
            assert d1.format(~"%F") == d2.format(~"%F");
        }
    }    
}


#[test]
pub fn test_clone() {
    let mut temp: datetime;
    match new_from_unix_utc(0) {
        Err(s) => fail(s),
        Ok(t) => {
            temp = t.clone();
            assert temp.format(~"%F") == temp.clone().format(~"%F");
        }
    }
    
    for 100.times {
        temp = temp.clone();
    }
}

#[test]
pub fn datetime_new() {
    // this will probably fail if the method for constructing types changes.
    new_from_unix_utc(0);
}




