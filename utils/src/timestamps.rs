use time::{self, OffsetDateTime};

pub fn within_days(unix_timestamp: i64, days: i64) -> bool {
    let given_time = time::OffsetDateTime::from_unix_timestamp(unix_timestamp).unwrap();
    let now = time::OffsetDateTime::now_utc();
    let difference = now - given_time;

    difference <= time::Duration::days(days)
}

pub fn difference_in_days(timestamp1: i64, timestamp2: i64) -> i64 {
    let datetime1 = OffsetDateTime::from_unix_timestamp(timestamp1).expect("Invalid timestamp1");
    let datetime2 = OffsetDateTime::from_unix_timestamp(timestamp2).expect("Invalid timestamp2");

    // Calculate the difference and convert it to whole days
    let difference = datetime2 - datetime1;
    difference.whole_days()
}

pub fn get_unix_timestamp() -> i64 { OffsetDateTime::now_utc().unix_timestamp() }
