use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::channel::Channel;

lazy_static! {
    // a collection of global static message queue groups.
    // k: group id v: group
    pub static ref GROUPS: Arc<RwLock<HashMap<String, Group>>> =
        Arc::new(RwLock::new(HashMap::<String, Group>::default()));
}

pub struct Groups;
impl Groups {
    pub fn is(group_id: String) {}
    pub fn insert() {}
}

/// message queue group, the same topic in the same group shares a message channel.
pub struct Group {
    // group id
    id: String,
    // k: topic v: the message channel corresponding to the topic
    channels: Arc<RwLock<HashMap<String, Channel>>>,
}

impl Group {
    pub fn new(id: String) -> Self {
        Self {
            id: id,
            channels: Arc::new(RwLock::new(HashMap::<String, Channel>::default())),
        }
    }
}
