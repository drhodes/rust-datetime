use std;
import datetime::*;
import timezone::*;

// check to see if your timezone is correct.
// #[test]
// fn test_timezone() {
//     let t = timezone_new_local();
//     log(error, t.is_dst(0)); // is this right?
//     log(error, t.get_abbreviation(1));
// }

#[test]
fn test_adders() {
    alt new_from_unix_utc(0) {
      err(s) {
        fail(s);
      }
      ok(t) {
        assert t.format(~"%c") == ok(~"Thu Jan  1 00:00:00 1970");
        assert t.equal(t) == true;
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
fn test_clone() {
    let mut temp: datetime;
    alt new_from_unix_utc(0) {
      err(s) {
        fail(s)
      }
      
      ok(t) {
        temp = t.clone();
        assert temp.format(~"%F") == temp.clone().format(~"%F");
      }
    }
    
     for 100.times {
         temp = temp.clone();
     }
}

#[test]
fn test_adders() {
    let t = new_from_unix_utc(0);
    assert t.format(~"%c") == ok(~"Thu Jan  1 00:00:00 1970");
    assert t.equal(t) == true;
    assert t.equal(t.add_years(1)) == false;
    assert t.equal(t.add_seconds(0.001 as f64)) == false;
    assert t.add_seconds(86400 as f64).equal(t.add_days(1));

}


fn id(dt: datetime) -> datetime {    
    dt.clone()
}

#[test]
fn test_clone() {
    let mut temp = new_from_unix_utc(0);
    assert temp.format(~"%F") == temp.clone().format(~"%F");

    let mut count = 0;
    while count < 100 {
        temp = temp.clone();
        count += 1
    }       
}