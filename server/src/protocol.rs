/// communication protocol
use bincode::{Decode, Encode, config};
use std::mem;

use crate::msg::Message;

// fixed protocol identifier
const PROTOCOL_IDENTIFIER: &str = "luminmq";

/// protocol header
#[derive(Encode, Decode, PartialEq, Debug)]
pub struct ProtocolHead {
    // fixed-length protocol identifier.
    identifier: String,
    // the byte size of the data area
    data_size: u32,
}

impl ProtocolHead {
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
    pub fn size() -> usize {
        let len = PROTOCOL_IDENTIFIER.len();
        let identifier_size = len * mem::size_of::<u8>();
        let data_area_size = mem::size_of::<u32>();
        identifier_size + data_area_size
    }
}

pub struct Protocol {
    protocol_head: ProtocolHead,
    message: Message,
}

impl Protocol {
    // bytes size = protocol header size + data area size.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            protocol_head: todo!(),
            message: todo!(),
        }
    }
}
