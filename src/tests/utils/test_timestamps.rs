use time::{ Duration, OffsetDateTime };

use crate::utils::timestamps::{ self, difference_in_days, within_days };

#[test]
fn test_within_days_true() {
    // Simulate a timestamp 1 day ago
    let one_day_ago = OffsetDateTime::now_utc() - Duration::days(1);
    assert!(within_days(one_day_ago.unix_timestamp(), 2));
}

#[test]
fn test_within_days_false() {
    // Simulate a timestamp 3 days ago
    let three_days_ago = OffsetDateTime::now_utc() - Duration::days(3);
    assert!(!within_days(three_days_ago.unix_timestamp(), 2));
}

#[test]
fn test_between_two_timestamps_true() {
    let two_days_ago = 1712071745;
    let now = 1711898945;
    let days = difference_in_days(now, two_days_ago);
    assert!(days == 2);
}

#[test]
fn test_same_day_true() {
    let two_hours_ago = OffsetDateTime::now_utc() - Duration::hours(2);
    let now = timestamps::get_unix_timestamp();
    let days = difference_in_days(now, two_hours_ago.unix_timestamp());
    assert!(days == 0);
}

#[test]
fn test_within_days_edge_case_true() {
    // Edge case: exactly the number of days specified
    let two_days_ago = OffsetDateTime::now_utc() - Duration::days(2) + Duration::seconds(1);
    assert!(within_days(two_days_ago.unix_timestamp(), 2));
}

#[test]
fn test_within_days_edge_case_false() {
    // Edge case: just over the number of days specified
    let slightly_more_than_two_days_ago =
        OffsetDateTime::now_utc() - Duration::days(2) - Duration::seconds(1);
    assert!(!within_days(slightly_more_than_two_days_ago.unix_timestamp(), 2));
}
