use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::channel::{self, Channel, ChannelMode};

lazy_static! {
    // a collection of global static message queue groups.
    // k: group id v: group
    pub static ref GROUPS: Arc<RwLock<HashMap<String, Group>>> =
        Arc::new(RwLock::new(HashMap::<String, Group>::default()));
}

/// Specify the consumption mode within the same group.
pub enum GroupMode {
    // Multiple consumers in the same consumer group process messages sequentially, ensuring that each message is processed only once.
    Cluster,
    // All consumers in the same consumer group process the same message, and each message is consumed multiple times.
    Broadcast,
}

pub struct Groups;
impl Groups {
    pub fn insert(group_id: String, group_mode: GroupMode) {
        GROUPS.write().unwrap().insert(
            group_id.to_string(),
            Group::new(group_id.to_string(), group_mode),
        );
        // start message group
        GROUPS
            .read()
            .unwrap()
            .get(&group_id.clone())
            .unwrap()
            .start();
    }
}

/// message queue group, the same topic in the same group shares a message channel.
pub struct Group {
    // group id
    id: String,
    // k: topic v: the message channel corresponding to the topic
    channels: Arc<RwLock<HashMap<String, Channel>>>,
    mode: GroupMode,
}

impl Group {
    pub fn new(id: String, mode: GroupMode) -> Self {
        Self {
            id: id,
            channels: Arc::new(RwLock::new(HashMap::<String, Channel>::default())),
            mode: mode,
        }
    }
    pub fn insert_channel(&self) {}
    pub fn remove_channel(&self) {}
    pub fn start(&self) {
        tokio::spawn(async move {});
    }
}
