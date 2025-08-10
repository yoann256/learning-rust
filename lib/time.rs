use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use std::time::Duration;



pub fn get_seconds_millis(elapsedTime: Duration) -> String {
    return format!("{}.{}", elapsedTime.as_secs(), elapsedTime.as_millis());
}