use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_nstime() -> u64 {

    let dur = 

    SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    dur.as_secs() << 30 | dur.subsec_nanos() as u64

}