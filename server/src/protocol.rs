use axum::body;
/// communication protocol
use bincode::{Decode, Encode, config};
use std::mem;

use crate::{
    msg::{Message, MessageDTO},
    tool::codec::{decode, encode},
};

// fixed protocol identifier
pub const PROTOCOL_IDENTIFIER: &str = "luminmq";

/// protocol header
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct ProtocolHead {
    // fixed-length protocol identifier.
    pub identifier: String,
    // the byte size of the data area
    pub data_size: u32,
}

impl ProtocolHead {
    // determine whether it is the luminmq communication protocol.
    pub fn is(bytes: &[u8]) -> bool {
        match ProtocolHead::from_bytes(bytes) {
            Ok(protocol_head) => {
                if protocol_head.identifier == PROTOCOL_IDENTIFIER {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }
    // get the byte size of the protocol header
    pub const fn size() -> usize {
        let len = PROTOCOL_IDENTIFIER.len();
        let identifier_size = len * mem::size_of::<u8>();
        let size = mem::size_of::<u32>();
        identifier_size + size
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        decode(bytes)
    }
    pub fn to_byte_vec(&self) -> Vec<u8> {
        encode(self)
    }
    pub fn set_data_size(&mut self, data_size: u32) {
        self.data_size = data_size;
    }
}

impl Default for ProtocolHead {
    fn default() -> Self {
        Self {
            identifier: PROTOCOL_IDENTIFIER.to_string(),
            data_size: 0,
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
struct ProtocolBody {
    pub message: Option<MessageDTO>,
}

impl ProtocolBody {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ()> {
        decode(bytes)
    }
}

impl Default for ProtocolBody {
    fn default() -> Self {
        Self {
            message: Some(MessageDTO::default()),
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Protocol {
    pub head: ProtocolHead,
    pub body: Option<ProtocolBody>,
}

impl Protocol {
    // verify protocol header
    pub fn verify_protocol_head(bytes: &[u8]) -> bool {
        match ProtocolHead::from_bytes(bytes) {
            Ok(head) => {
                if head.identifier.eq(PROTOCOL_IDENTIFIER) {
                    true
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }
    // build protocol head
    pub fn build_protocol_head_by_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        match ProtocolHead::from_bytes(bytes) {
            Ok(head) => {
                self.head = head;
                return self;
            }
            Err(_) => return self,
        }
    }
    // build protocol body
    pub fn build_protocol_body_by_bytes(&mut self, bytes: &[u8]) -> &mut Self {
        match ProtocolBody::from_bytes(bytes) {
            Ok(body) => {
                self.body = Some(body);
                return self;
            }
            Err(_) => return self,
        }
    }
    // to byte vec
    pub fn to_byte_vec(&self) -> Vec<u8> {
        encode(self)
    }
    // protocol body size
    // when calling this function, the protocol header should have been initialized.
    pub fn protocol_body_size(&self) -> u32 {
        self.head.data_size
    }
    // perform preparations before transmission.
    pub fn ready(&mut self) -> Result<&mut Self, ()> {
        if self.body.is_none() {
            Err(())
        } else if self.head.data_size == 0 {
            let size = self.body.clone().unwrap().message.unwrap().size();
            self.head.set_data_size(size as u32);
            Ok(self)
        } else {
            Ok(self)
        }
    }
    // get message
    pub fn get_message(&self) -> Result<Message, ()> {
        match &self.body {
            Some(body) => match &body.message {
                Some(m) => Ok(m.to_message()),
                None => Err(()),
            },
            None => Err(()),
        }
    }
    // protocol head size
    pub fn protocol_head_size() -> usize {
        ProtocolHead::size()
    }
}

impl Default for Protocol {
    fn default() -> Protocol {
        Protocol {
            head: ProtocolHead::default(),
            body: Some(ProtocolBody::default()),
        }
    }
}
