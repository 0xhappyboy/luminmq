use std::{
    collections::HashMap,
    mem,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::{
    channel::{self, Channel, ChannelMode},
    msg::Message,
};

lazy_static! {
    // a collection of global static message queue groups.
    // k: group id v: group
    static ref GROUPS: Arc<RwLock<HashMap<String, Arc<RwLock<Group>>>>> =
        Arc::new(RwLock::new(HashMap::<String, Arc<RwLock<Group>>>::default()));
}

/// Specify the consumption mode within the same group.
#[derive(Debug, Clone, Copy)]
pub enum GroupMode {
    // Multiple consumers in the same consumer group process messages sequentially, ensuring that each message is processed only once.
    Cluster,
    // All consumers in the same consumer group process the same message, and each message is consumed multiple times.
    Broadcast,
}

pub struct Groups;
impl Groups {
    pub fn insert(group_id: String, group: Group) {
        GROUPS
            .write()
            .unwrap()
            .insert(group_id.to_string(), Arc::new(RwLock::new(group)));
        // start message group
        Groups::get_group_by_id(group_id)
            .unwrap()
            .read()
            .unwrap()
            .start();
    }
    // insert message
    pub fn insert_message(group_id: String, topic: String, message: Message) {
        let group = Groups::get_group_by_id(group_id).unwrap();
        group.write().unwrap().insert_message(topic, message);
    }
    pub fn group_num() -> u64 {
        GROUPS.write().unwrap().len().try_into().unwrap()
    }
    pub fn contains_id(grou_id: String) -> bool {
        GROUPS.write().unwrap().contains_key(&grou_id)
    }
    pub fn get_group_by_id(grou_id: String) -> Option<Arc<RwLock<Group>>> {
        if Groups::contains_id(grou_id.clone()) {
            Some(Arc::clone(GROUPS.write().unwrap().get(&grou_id).unwrap()))
        } else {
            None
        }
    }
}

/// message queue group, the same topic in the same group shares a message channel.
pub struct Group {
    // group id
    id: String,
    // k: topic v: the message channel corresponding to the topic
    channels: Arc<RwLock<HashMap<String, Arc<RwLock<Channel>>>>>,
    mode: GroupMode,
}

impl Group {
    pub fn new(id: String, mode: GroupMode) -> Self {
        Self {
            id: id,
            channels: Arc::new(RwLock::new(
                HashMap::<String, Arc<RwLock<Channel>>>::default(),
            )),
            mode: mode,
        }
    }
    pub fn insert_channel(&self, topic: String, mode: ChannelMode) {
        self.channels.write().unwrap().insert(
            topic.clone(),
            Arc::new(RwLock::new(Channel::new(
                topic.to_string(),
                self.id.clone(),
                mode,
                self.mode.clone(),
            ))),
        );
        self.get_channel(topic.clone())
            .unwrap()
            .read()
            .unwrap()
            .start();
    }
    pub fn remove_channel(&self) {}
    pub fn get_channel(&self, topic: String) -> Option<Arc<RwLock<Channel>>> {
        if self.contains_channel(topic.clone()) {
            Some(Arc::clone(
                self.channels.read().unwrap().get(&topic).unwrap(),
            ))
        } else {
            None
        }
    }
    pub fn contains_channel(&self, topic: String) -> bool {
        self.channels.write().unwrap().contains_key(&topic.clone())
    }
    pub fn insert_message(&mut self, topic: String, message: Message) {
        match self.get_channel(topic.clone()) {
            Some(channel) => {
                channel.write().unwrap().enqueue(message);
            }
            None => (),
        }
    }
    pub fn start(&self) {
        let id = self.id.clone();
        let mode = self.mode.clone();
        let channel = Arc::clone(&self.channels);
        tokio::spawn(async move {
            loop {
                thread::sleep(Duration::from_millis(1000));
                match mode {
                    GroupMode::Cluster => {}
                    GroupMode::Broadcast => {}
                }
            }
        });
    }
}
