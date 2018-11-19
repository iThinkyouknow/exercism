extern crate chrono;
use chrono::{DateTime, Utc, TimeZone};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    Utc.timestamp(start.timestamp() + 10i64.pow(9), 0)
}
