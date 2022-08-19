use std::time;
use std::time::Duration;
use chrono::{DateTime, Local, Locale, NaiveDate, TimeZone, Utc};

#[test]
fn time() {
    let date = Local.ymd(2022, 8, 18).and_hms(0,0,0).to_string();
    let time = date.parse::<DateTime<Local>>().unwrap();
}