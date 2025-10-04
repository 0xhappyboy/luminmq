use bincode::{Decode, Encode, config};
use std::mem;

use crate::msg::Message;

/// communication protocol

const PROTOCOL_IDENTIFIER: &str = "luminmq";

/// protocol header
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct ProtocolHead {
    // Fixed-length protocol identifier.
    identifier: String,
    // data size
    data_size: u32,
}

impl ProtocolHead {
    pub fn new() -> Self {
        Self {
            identifier: PROTOCOL_IDENTIFIER.to_string(),
            data_size: 1024,
        }
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let head: ProtocolHead = unsafe { std::ptr::read(bytes.as_ptr() as *const _) };
        head
    }
    // determine whether it is the luminmq communication protocol.
    pub fn is(bytes: &[u8]) -> bool {
        let head = ProtocolHead::from_bytes(bytes);
        if head.identifier == PROTOCOL_IDENTIFIER {
            true
        } else {
            false
        }
    }
    // get the byte size of the protocol header
    pub fn size(&self) -> usize {
        let len = self.identifier.len();
        let size_of_data = len * mem::size_of::<u8>();
        size_of_data
    }
}

pub struct Protocol {
    protocol_head: ProtocolHead,
    message: Message,
}

impl Protocol {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            protocol_head: (),
            message: (),
        }
    }
}
