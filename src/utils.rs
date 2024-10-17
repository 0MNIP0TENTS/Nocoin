// src/utils.rs

pub fn current_timestamp() -> u64 {
    // Returns the current Unix timestamp
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
