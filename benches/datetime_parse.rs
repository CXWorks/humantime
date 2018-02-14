#![feature(test)]
extern crate chrono;
extern crate humantime;
extern crate test;

use chrono::{DateTime};
use humantime::parse_rfc3339_seconds;


#[bench]
fn rfc3339_seconds_humantime(b: &mut test::Bencher) {
    b.iter(|| {
        parse_rfc3339_seconds("2018-02-13T23:08:32Z").unwrap()
    });
}

#[bench]
fn datetime_utc_parse(b: &mut test::Bencher) {
    b.iter(|| {
        DateTime::parse_from_rfc3339("2018-02-13T23:08:32Z").unwrap()
    });
}
