use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;
use mio::{Token, net::TcpStream};
use rand::seq::IndexedRandom;

use crate::tool::common::get_keys_for_value;

lazy_static! {
      // consumer binder.
    // k: (group id,topic) v: function name
    static ref CONSUMER_BINDER: Mutex<HashMap< (String, String), String>> = Mutex::new(HashMap::< (String, String), String>::default());
    // connection pool
    static ref CONNECTION_POOL: Mutex<HashMap<Token, Mutex<TcpStream>>> = Mutex::new(HashMap::<Token, Mutex<TcpStream>>::default());
    // connection pool and gourp bind
    // k: token v: (group id, topic)
    static ref CONNECTION_POOL_GROUP_BIND: Mutex<HashMap<Token, (String, String)>> = Mutex::new(HashMap::<Token, (String, String)>::default());
}

pub struct ConsumerBinder;
impl ConsumerBinder {
    pub fn insert(k: (String, String), v: String) {
        CONSUMER_BINDER.lock().unwrap().insert(k, v);
    }
    pub fn get(k: (String, String)) -> Result<String, String> {
        let m = CONSUMER_BINDER.lock().unwrap();
        if m.contains_key(&k) {
            return Ok(m.get(&k).unwrap().to_string());
        } else {
            return Err("key does not exist.".to_string());
        }
    }
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
    pub fn get_random_token(v: (String, String)) -> Option<Token> {
        let token_list = get_keys_for_value(CONNECTION_POOL_GROUP_BIND.lock().unwrap(), v);
        token_list.choose(&mut rand::rng()).copied()
    }
    pub fn insert(k: Token, v: (String, String)) {
        CONNECTION_POOL_GROUP_BIND.lock().unwrap().insert(k, v);
    }
}
