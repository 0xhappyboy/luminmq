use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use tokio::stream;

use crate::{
    group::GroupMode,
    msg::{ConsumerType, Message, MessageType},
    protocol::Protocol,
    tool::common::get_keys_for_value,
    topic::Topic,
    types::{CONNECTION_POOL, CONNECTION_POOL_GROUP_BIND},
};

/// Consumption mode for messages within a channel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChannelMode {
    // The server actively pushes the message in the channel to the consumer.
    Push,
    // The server waits for the consumer to pull data from the channel.
    Pull,
    // none
    None,
}

/// message channel
pub struct Channel {
    pub topic: Topic,
    pub group_id: String,
    pub mode: ChannelMode,
    group_mode: GroupMode,
    message_queue: Arc<RwLock<Queue>>,
}
impl Channel {
    pub fn new(topic: String, group_id: String, mode: ChannelMode, group_mode: GroupMode) -> Self {
        Self {
            message_queue: Arc::new(RwLock::new(Queue::new())),
            mode: mode,
            group_mode: group_mode,
            topic: Topic::new(topic),
            group_id: group_id,
        }
    }
    // first in
    pub fn enqueue(&mut self, message: Message) {
        self.message_queue.write().unwrap().enqueue(message);
    }
    // first out
    pub fn dequeue(&mut self) -> Option<Message> {
        self.message_queue.write().unwrap().dequeue()
    }
    // is empty
    pub fn is_empty(&self) -> bool {
        self.message_queue.read().unwrap().is_empty()
    }
    // message num
    pub fn message_num(&self) -> u64 {
        self.message_queue.read().unwrap().len().try_into().unwrap()
    }
    pub fn start(&self) {
        let group_id = self.group_id.clone();
        let topic = self.topic.name.clone();
        let channel_mode = self.mode.clone();
        let queue = Arc::clone(&self.message_queue);
        let group_mode = self.group_mode.clone();
        tokio::spawn(async move {
            loop {
                // The mode of the group to which the current pipeline belongs
                match group_mode {
                    GroupMode::Broadcast => match channel_mode {
                        ChannelMode::Push => {
                            // test
                            thread::sleep(Duration::from_millis(1000));
                            let token_list = get_keys_for_value(
                                CONNECTION_POOL_GROUP_BIND.lock().unwrap(),
                                (group_id.clone(), topic.clone()),
                            );
                            token_list.iter().for_each(|token| {
                                match CONNECTION_POOL.lock().unwrap().get(token) {
                                    Some(stream) => {
                                        // test protocol
                                        let mut protocol = &mut Protocol::default();
                                        let mut msg = Message::default();
                                        msg.group_id = "group-test".to_string();
                                        msg.topic = Topic::new("topic-test".to_string());
                                        msg.consumer_type = ConsumerType::Send;
                                        msg.msg_type = MessageType::Business;
                                        protocol.insert_message(msg.to_messagedto());
                                        protocol.ready();
                                        Protocol::writer(stream, protocol);
                                    }
                                    None => {
                                        // No connection source exists
                                    }
                                }
                            });
                        }
                        ChannelMode::Pull => {}
                        ChannelMode::None => todo!(),
                    },
                    GroupMode::Cluster => match channel_mode {
                        ChannelMode::Push => {
                            thread::sleep(Duration::from_millis(1000));
                        }
                        ChannelMode::Pull => {}
                        ChannelMode::None => todo!(),
                    },
                }
            }
        });
    }
}

struct Queue {
    queue: VecDeque<Message>,
}
impl Queue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
    // first in
    pub fn enqueue(&mut self, message: Message) {
        self.queue.push_back(message);
    }
    // first out
    pub fn dequeue(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }
    // is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    // len
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}
impl Default for Queue {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}
