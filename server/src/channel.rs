use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use crate::msg::Message;

/// Consumption mode for messages within a channel.
pub enum ChannelMode {
    // The server actively pushes the message in the channel to the consumer.
    Push,
    // The server waits for the consumer to pull data from the channel.
    Pull,
}

/// message channel
pub struct Channel {
    message_queue: VecDeque<Message>,
    mode: ChannelMode,
}
impl Channel {
    pub fn new(mode: ChannelMode) -> Self {
        Self {
            message_queue: VecDeque::new(),
            mode: mode,
        }
    }
    // first in
    fn enqueue(&mut self, message: Message) {
        self.message_queue.push_back(message);
    }
    // first out
    fn dequeue(&mut self) -> Option<Message> {
        self.message_queue.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.message_queue.is_empty()
    }
}
