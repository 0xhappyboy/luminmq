use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use mio::{Token, net::TcpStream};

use crate::tool::common::get_keys_for_value;

lazy_static! {
    // connection pool
    static ref CONNECTION_POOL: Mutex<HashMap<Token, Mutex<TcpStream>>> = Mutex::new(HashMap::<Token, Mutex<TcpStream>>::default());
    // connection pool and gourp bind
    // k: token v: (group id, topic)
    static ref CONNECTION_POOL_GROUP_BIND: Mutex<HashMap<Token, (String, String)>> = Mutex::new(HashMap::<Token, (String, String)>::default());
}

pub struct ConnectionPool;
impl ConnectionPool {
    // Handle the connection source of the specified token
    pub fn handle(token: &Token, handler: impl Fn(&mut TcpStream)) {
        match CONNECTION_POOL.lock().unwrap().get_mut(token) {
            Some(stream) => {
                handler(&mut stream.lock().unwrap());
            }
            None => {
                // No connection source exists
            }
        }
    }
    pub fn insert(k: Token, v: TcpStream) {
        CONNECTION_POOL.lock().unwrap().insert(k, Mutex::new(v));
    }
    pub fn remove(k: Token) {
        CONNECTION_POOL.lock().unwrap().remove(&k);
    }
    pub fn writer(group_id: String, topic: String) {}
}

pub struct ConnectionPoolAndGroupBind;
impl ConnectionPoolAndGroupBind {
    pub fn get_token_list(v: (String, String)) -> Vec<Token> {
        let token_list = get_keys_for_value(CONNECTION_POOL_GROUP_BIND.lock().unwrap(), v);
        token_list
    }
    pub fn insert(k: Token, v: (String, String)) {
        CONNECTION_POOL_GROUP_BIND.lock().unwrap().insert(k, v);
    }
}
