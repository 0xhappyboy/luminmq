use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;

use crate::{
    channel::{Channel, ChannelMode},
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
    pub fn default_insert_group(group_id: String, mode: GroupMode) {
        let group = Group::new(group_id.clone(), mode);
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
    pub fn insert_group(group_id: String, group: Group) {
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
        if Groups::contains_topic(group_id.clone(), topic.clone()) {
            let group = Groups::get_group_by_id(group_id.clone()).unwrap();
            group
                .write()
                .unwrap()
                .insert_message(topic.clone(), message);
        }
    }
    pub fn group_num() -> u64 {
        GROUPS.write().unwrap().len().try_into().unwrap()
    }
    pub fn get_message_num_by_topic(group_id: String, topic: String) -> u64 {
        if Groups::contains_id(group_id.clone()) {
            let group = Groups::get_group_by_id(group_id);
            let message_num = group
                .unwrap()
                .read()
                .unwrap()
                .get_channel(topic)
                .unwrap()
                .read()
                .unwrap()
                .message_num();
            message_num
        } else {
            0
        }
    }
    pub fn contains_id(group_id: String) -> bool {
        GROUPS.write().unwrap().contains_key(&group_id)
    }
    pub fn contains_topic(group_id: String, topic: String) -> bool {
        if Groups::contains_id(group_id.clone()) {
            let group_id = Arc::clone(&Groups::get_mut_group_by_id(group_id.clone()).unwrap());
            group_id.read().unwrap().contains_channel(topic.clone())
        } else {
            false
        }
    }
    pub fn get_group_by_id(grou_id: String) -> Option<Arc<RwLock<Group>>> {
        if Groups::contains_id(grou_id.clone()) {
            Some(Arc::clone(GROUPS.write().unwrap().get(&grou_id).unwrap()))
        } else {
            None
        }
    }
    pub fn get_mut_group_by_id(group_id: String) -> Option<Arc<RwLock<Group>>> {
        if Groups::contains_id(group_id.clone()) {
            Some(Arc::clone(
                GROUPS.write().unwrap().get_mut(&group_id).unwrap(),
            ))
        } else {
            None
        }
    }
    pub fn insert_channel(group_id: String, topic: String, channel_mode: ChannelMode) {
        if Groups::contains_id(group_id.clone()) {
            let group = Arc::clone(&Groups::get_mut_group_by_id(group_id.clone()).unwrap());
            if !group.read().unwrap().contains_channel(topic.clone()) {
                group
                    .write()
                    .unwrap()
                    .insert_channel(topic.clone(), channel_mode);
            }
        }
    }
    pub fn get_channel_mode(grou_id: String, topic: String) -> ChannelMode {
        if Groups::contains_id(grou_id.clone()) {
            let group = Groups::get_group_by_id(grou_id);
            let channel_mode = group
                .unwrap()
                .read()
                .unwrap()
                .get_channel(topic)
                .unwrap()
                .read()
                .unwrap()
                .mode;
            channel_mode
        } else {
            ChannelMode::None
        }
    }
    // get a message from the channel.
    pub fn get_a_message(grou_id: String, topic: String) -> Result<Message, ()> {
        if Groups::contains_id(grou_id.clone()) {
            let group = Arc::clone(&Groups::get_group_by_id(grou_id).unwrap());
            if group.read().unwrap().contains_channel(topic.clone()) {
                match group
                    .write()
                    .unwrap()
                    .get_channel(topic.clone())
                    .unwrap()
                    .write()
                    .unwrap()
                    .dequeue()
                {
                    Some(msg) => {
                        return Ok(msg);
                    }
                    None => Err(()),
                }
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

/// message queue group, the same topic in the same group shares a message channel.
struct Group {
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
    pub fn remove_channel(&self, topic: String) {
        self.channels.write().unwrap().remove(&topic);
    }
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
        let _id = self.id.clone();
        let mode = self.mode.clone();
        let _channel = Arc::clone(&self.channels);
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
