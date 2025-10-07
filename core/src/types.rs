use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use mio::{Token, net::TcpStream};

lazy_static! {
    // connection pool
    pub static ref CONNECTION_POOL: Mutex<HashMap<Token, TcpStream>> = Mutex::new(HashMap::<Token, TcpStream>::default());
    // connection pool and gourp bind
    // k: token v: (group id, topic)
    pub static ref CONNECTION_POOL_GROUP_BIND: Mutex<HashMap<Token, (String, String)>> = Mutex::new(HashMap::<Token, (String, String)>::default());
}
