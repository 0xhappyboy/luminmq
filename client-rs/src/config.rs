use std::sync::Mutex;

use lazy_static::lazy_static;

lazy_static! {
    // message queue server listener port
    pub static ref LISTENER_PORT: Mutex<String> = Mutex::new("127.0.0.1:8080".to_string());
}
