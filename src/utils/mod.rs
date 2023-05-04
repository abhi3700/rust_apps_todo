use std::time::{SystemTime, UNIX_EPOCH};

fn get_current_time() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
}
