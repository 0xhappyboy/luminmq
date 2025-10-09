use std::io::Write;

use bincode::{Decode, Encode, error::DecodeError};
use mio::net::TcpStream;

use crate::{
    channel::ChannelMode,
    group::Groups,
    protocol::Protocol,
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

/// message type
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub enum MessageStatus {
    // success
    Success,
    // fail
    Fail,
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
    // message status
    // 0: success
    // 1: fail
    // 2: none
    status: u16,
    // business data. If msg_type is 0, this field may be empty.
    data: String,
}

impl MessageDTO {
    pub fn new(
        group_id: String,
        topic: String,
        msg_type: u16,
        consumer_type: u16,
        status: u16,
        data: String,
    ) -> Self {
        Self {
            group_id: group_id,
            topic: topic,
            msg_type: msg_type,
            consumer_type: consumer_type,
            status: status,
            data: data,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        decode(bytes)
    }
    pub fn to_byte_vec(&self) -> Vec<u8> {
        encode(self)
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
            status: if self.status == 0 {
                MessageStatus::Success
            } else if self.status == 1 {
                MessageStatus::Fail
            } else {
                MessageStatus::None
            },
        }
    }
}

impl Default for MessageDTO {
    fn default() -> Self {
        Self {
            group_id: "".to_string(),
            topic: "".to_string(),
            msg_type: 0,
            consumer_type: 0,
            status: 0,
            data: "".to_string(),
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Message {
    pub group_id: String,
    pub topic: Topic,
    pub data: String,
    pub msg_type: MessageType,
    pub consumer_type: ConsumerType,
    pub status: MessageStatus,
}

impl Message {
    pub fn new(
        group_id: String,
        topic: String,
        data: String,
        msg_type: MessageType,
        consumer_type: ConsumerType,
        status: MessageStatus,
    ) -> Self {
        Self {
            group_id: group_id,
            topic: Topic { name: topic },
            data: data,
            msg_type: msg_type,
            consumer_type: consumer_type,
            status: status,
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
            match self.status {
                MessageStatus::Success => 0,
                MessageStatus::Fail => 1,
                MessageStatus::None => 2,
            },
            self.data.to_string(),
        )
    }
    /// message handle
    pub fn handle(&mut self, stream: &TcpStream) {
        match self.msg_type {
            MessageType::System => {
                // system message
                todo!()
            }
            MessageType::Business => match self.consumer_type {
                ConsumerType::Pull => {
                    if ChannelMode::Pull.eq(&Groups::get_channel_mode(
                        self.group_id.clone(),
                        self.topic.name.clone(),
                    )) {
                        // handle ..
                        match Groups::get_a_message(
                            self.group_id.to_string(),
                            self.topic.name.to_string(),
                        ) {
                            Ok(mut msg) => {
                                msg.status = MessageStatus::Success;
                                let _ = msg.writer(stream);
                            }
                            Err(_) => {
                                self.status = MessageStatus::Fail;
                                self.data = "No message exists.".to_string();
                                let _ = self.writer(stream);
                            }
                        }
                    }
                }
                ConsumerType::Send => {
                    // the consumer inserts a new message.
                    Groups::insert_message(
                        self.group_id.clone(),
                        self.topic.name.clone(),
                        self.clone(),
                    );
                }
                ConsumerType::None => {}
            },
            MessageType::None => (),
        }
    }
    // massge writer
    pub fn writer(&self, mut stream: &TcpStream) -> Result<usize, String> {
        let protocol = &mut Protocol::default();
        protocol.insert_message(self.to_messagedto());
        let _ = protocol.ready();
        let protocol_buf = protocol.to_byte_vec();
        match stream.write(&protocol_buf) {
            Ok(size) => {
                return Ok(size);
            }
            Err(e) => {
                return Err(format!("{:?}", e));
            }
        }
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            group_id: "".to_string(),
            topic: Topic::default(),
            data: "".to_string(),
            msg_type: MessageType::None,
            consumer_type: ConsumerType::None,
            status: MessageStatus::None,
        }
    }
}
