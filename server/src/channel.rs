use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::{
    group::{self, GroupMode},
    msg::Message,
    server::CONNECTION_POOL_GROUP_BIND,
    tool::common::get_keys_for_value,
    topic::Topic,
};

/// Consumption mode for messages within a channel.
#[derive(Debug, Clone, Copy)]
pub enum ChannelMode {
    // The server actively pushes the message in the channel to the consumer.
    Push,
    // The server waits for the consumer to pull data from the channel.
    Pull,
}

/// message channel
pub struct Channel {
    topic: Topic,
    group_id: String,
    mode: ChannelMode,
    group_mode: GroupMode,
    message_queue: Arc<RwLock<VecDeque<Message>>>,
}
impl Channel {
    pub fn new(topic: String, group_id: String, mode: ChannelMode, group_mode: GroupMode) -> Self {
        Self {
            message_queue: Arc::new(RwLock::new(VecDeque::new())),
            mode: mode,
            group_mode: group_mode,
            topic: Topic::new(topic),
            group_id: group_id,
        }
    }
    // first in
    pub fn enqueue(&mut self, message: Message) {
        self.message_queue.write().unwrap().push_back(message);
    }
    // first out
    pub fn dequeue(&mut self) -> Option<Message> {
        self.message_queue.write().unwrap().pop_front()
    }
    // is empty
    pub fn is_empty(&self) -> bool {
        self.message_queue.read().unwrap().is_empty()
    }
    pub fn start(&self) {
        let group_id = self.group_id.clone();
        let topic = self.topic.clone();
        let channel_mode = self.mode.clone();
        let queue = Arc::clone(&self.message_queue);
        let group_mode = self.group_mode.clone();
        tokio::spawn(async move {
            loop {
                // The mode of the group to which the current pipeline belongs
                match group_mode {
                    GroupMode::Broadcast => match channel_mode {
                        ChannelMode::Push => {
                            thread::sleep(Duration::from_millis(1000));
                            // get all connection pool token by (group id, topic)
                            let token_vec = get_keys_for_value(
                                CONNECTION_POOL_GROUP_BIND.lock().unwrap(),
                                (group_id.clone(), topic.name.clone()),
                            );
                            // get all connection by token
                            token_vec.iter().for_each(|token| {});
                        }
                        ChannelMode::Pull => {}
                    },
                    GroupMode::Cluster => match channel_mode {
                        ChannelMode::Push => {
                            thread::sleep(Duration::from_millis(1000));
                        }
                        ChannelMode::Pull => {}
                    },
                }
            }
        });
    }
}
