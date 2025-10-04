use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    // max event poll capacity
    pub static ref MAX_EVENT_POLL_CAPACITY: Mutex<u16> = Mutex::new(1024);
    // message queue server listener port
    pub static ref LISTENER_PORT: Mutex<String> = Mutex::new("0.0.0.0:8080".to_string());
}
