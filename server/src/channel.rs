use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, RwLock},
};

use crate::msg::Message;

/// message channel
pub struct Channel {
    message_queue: VecDeque<Message>,
}
impl Channel {
    pub fn new() -> Self {
        Self {
            message_queue: VecDeque::new(),
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
