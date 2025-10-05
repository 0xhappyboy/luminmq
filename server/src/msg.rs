use std::mem;

use bincode::{Decode, Encode, config};

use crate::{
    tool::codec::{decode, encode},
    topic::Topic,
};

/// message type
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum MessageType {
    // system message
    System,
    // business message
    Business,
    // none
    None,
}

/// consumer type
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum ConsumerType {
    // The consumer attempts to pull data from the channel.
    // Whether it succeeds depends on whether the channel is in pull mode.
    Pull,
    // the consumer sends data to the server.
    Send,
    // none
    None,
}

/// Message struct used only for data transmission and serialization.
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct MessageDTO {
    // group id
    group_id: String,
    // topic
    topic: String,
    // message type
    // 0: system message, used to identify system actions, such as new consumer registration.
    // 1: business message, which is a general identifier that identifies normal message queue message activities.
    msg_type: u16,
    // consumer mode
    // 0: pull
    // 1: Send
    consumer_type: u16,
    // business data. If msg_type is 0, this field may be empty.
    data: String,
}

impl MessageDTO {
    pub fn new(
        group_id: String,
        topic: String,
        msg_type: u16,
        consumer_type: u16,
        data: String,
    ) -> Self {
        Self {
            group_id: group_id,
            topic: topic,
            msg_type: msg_type,
            consumer_type: consumer_type,
            data: data,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        decode(bytes)
    }
    pub fn to_byte_vec(&self) -> Vec<u8> {
        encode(self)
    }
    pub fn size(&self) -> usize {
        // message type field size
        let msg_type_size = mem::size_of::<u16>();
        // group id size
        let group_id_size = self.group_id.len() * mem::size_of::<u8>();
        // topic size
        let topic_size = self.topic.len() * mem::size_of::<u8>();
        // data size
        let data_size = self.data.len() * mem::size_of::<u8>();
        // total size
        msg_type_size + group_id_size + topic_size + data_size
    }
    pub fn to_message(&self) -> Message {
        Message {
            group_id: self.group_id.clone(),
            topic: Topic {
                name: self.topic.clone(),
            },
            data: self.data.clone(),
            msg_type: if self.msg_type == 0 {
                MessageType::System
            } else if self.msg_type == 1 {
                MessageType::Business
            } else {
                MessageType::None
            },
            consumer_type: if self.consumer_type == 0 {
                ConsumerType::Pull
            } else if self.consumer_type == 1 {
                ConsumerType::Send
            } else {
                ConsumerType::None
            },
        }
    }
}

impl Default for MessageDTO {
    fn default() -> Self {
        Self {
            group_id: "group-test".to_string(),
            topic: "topic-test".to_string(),
            msg_type: 1,
            consumer_type: 1,
            data: "test data".to_string(),
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Message {
    group_id: String,
    topic: Topic,
    data: String,
    msg_type: MessageType,
    consumer_type: ConsumerType,
}

impl Message {
    pub fn new(
        group_id: String,
        topic: String,
        data: String,
        msg_type: MessageType,
        consumer_type: ConsumerType,
    ) -> Self {
        Self {
            group_id: group_id,
            topic: Topic { name: topic },
            data: data,
            msg_type: msg_type,
            consumer_type: consumer_type,
        }
    }
    pub fn is_group_id_empty(&self) -> bool {
        self.group_id.is_empty()
    }
    pub fn is_topic_id_empty(&self) -> bool {
        self.topic.is_name_empty()
    }
    pub fn to_messagedto(&self) -> MessageDTO {
        MessageDTO::new(
            self.group_id.to_string(),
            self.topic.name.to_string(),
            match self.msg_type {
                MessageType::System => 0,
                MessageType::Business => 1,
                MessageType::None => 2,
            },
            match self.consumer_type {
                ConsumerType::Pull => 0,
                ConsumerType::Send => 1,
                ConsumerType::None => 2,
            },
            self.data.to_string(),
        )
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            group_id: "default-group-1".to_string(),
            topic: Topic::default(),
            data: "default data".to_string(),
            msg_type: MessageType::Business,
            consumer_type: ConsumerType::Send,
        }
    }
}
