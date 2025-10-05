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
    pub fn new() -> Self {
        Self {
            identifier: PROTOCOL_IDENTIFIER.to_string(),
            data_size: 1024,
        }
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
    pub const fn size() -> usize {
        let len = PROTOCOL_IDENTIFIER.len();
        let identifier_size = len * mem::size_of::<u8>();
        let size = mem::size_of::<ProtocolHead>();
        identifier_size + size
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let config = config::standard();
        let (decoded, len): (ProtocolHead, usize) =
            bincode::decode_from_slice(&bytes[..], config).unwrap();
        decoded
    }
    pub fn to_byte_vec(&self) -> Vec<u8> {
        let config = config::standard();
        let bytes: Vec<u8> = bincode::encode_to_vec(&self, config).unwrap();
        bytes
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
