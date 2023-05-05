//! get current system time
//! Usage: externally during App FE during task creation
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) fn get_current_time() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}
