use lazy_static::lazy_static;

lazy_static! {
    pub static ref LISTENER_PORT: String = "0.0.0.0:8080".to_string();
}
