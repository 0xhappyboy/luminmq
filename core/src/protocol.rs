/// communication protocol
use bincode::{Decode, Encode, error::DecodeError};
use mio::net::TcpStream;
use std::io::{BufRead, BufReader, Write};

use crate::{
    msg::{Message, MessageDTO},
    tool::codec::{decode, encode, serialized_size},
};

// fixed protocol identifier
pub const PROTOCOL_IDENTIFIER: &str = "luminmq";

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Protocol {
    pub head: ProtocolHead,
    pub body: ProtocolBody,
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
    pub fn build_protocol_body_by_bytes(&mut self, bytes: &[u8]) -> Result<&mut Self, &mut Self> {
        match ProtocolBody::from_bytes(bytes) {
            Ok(body) => {
                self.body = body;
                return Ok(self);
            }
            Err(_e) => {
                return Ok(self);
            }
        }
    }
    // to byte vec
    pub fn to_byte_vec(&self) -> Vec<u8> {
        encode(self)
    }
    // protocol head size
    pub fn protocol_head_size() -> usize {
        ProtocolHead::size()
    }
    // protocol body size
    // when calling this function, the protocol header should have been initialized.
    pub fn protocol_body_size(&self) -> u32 {
        self.head.data_size
    }
    // perform preparations before transmission.
    pub fn ready(&mut self) -> Result<&mut Self, ()> {
        self.head
            .set_data_size(self.body.size().try_into().unwrap());
        Ok(self)
    }
    // get message
    pub fn get_message(&self) -> Result<Message, ()> {
        Ok(self.body.message.to_message())
    }
    // insert message
    pub fn insert_message(&mut self, message_dto: MessageDTO) {
        self.body.insert_message(message_dto);
    }
    // protocol reader
    pub fn reader(stream: &TcpStream) -> Result<Protocol, String> {
        let mut r: BufReader<&TcpStream> = BufReader::new(stream);
        loop {
            match r.fill_buf() {
                Ok(buf) => {
                    let protocol_head_size = Protocol::protocol_head_size();
                    let buf_1 = buf.len();
                    if buf_1 >= protocol_head_size {
                        let protocol_head_buf = &buf[..protocol_head_size];
                        if Protocol::verify_protocol_head(&protocol_head_buf) {
                            // build protocol
                            let mut protocol = &mut Protocol::default();
                            // build protocol head
                            protocol = protocol.build_protocol_head_by_bytes(&protocol_head_buf);
                            // consume system buffer
                            r.consume(protocol_head_size);
                            let protocol_body_size = protocol.head.data_size as usize;
                            if protocol_body_size > 0 {
                                match r.fill_buf() {
                                    Ok(buf) => {
                                        let buf_2 = buf.len();
                                        if buf_2 >= protocol_body_size {
                                            let protocol_body_buf = &buf[..protocol_body_size];
                                            match protocol
                                                .build_protocol_body_by_bytes(&protocol_body_buf)
                                            {
                                                Ok(protocol) => {
                                                    r.consume(protocol_body_size);
                                                    let p = protocol.clone();
                                                    return Ok(p);
                                                }
                                                Err(_) => {
                                                    return Err(
                                                        "protocol body serialization exception."
                                                            .to_string(),
                                                    );
                                                }
                                            }
                                        } else if buf_2 < protocol_body_size {
                                            r.consume(buf_2);
                                            return Err("dirty data.".to_string());
                                        } else if buf_2 == 0 {
                                            return Err("link is closed.".to_string());
                                        }
                                    }
                                    Err(_) => return Err("not enough data in buffer".to_string()),
                                }
                            }
                        } else {
                            return Err(
                                "the protocol header does not meet the requirements".to_string()
                            );
                        }
                    } else if buf_1 < protocol_head_size {
                        r.consume(buf_1);
                        return Err(
                            "dirty data, the buffer data does not meet the protocol header size."
                                .to_string(),
                        );
                    } else if buf_1 == 0 {
                        return Err("link is closed.".to_string());
                    }
                }
                Err(_) => return Err("buffer is empty".to_string()),
            }
        }
    }
    // protocol writer
    pub fn writer(mut stream: &TcpStream, protocol: &mut Protocol) {
        let protocol_buf = protocol.to_byte_vec();
        let _ = stream.write(&protocol_buf);
    }
}

impl Default for Protocol {
    fn default() -> Protocol {
        let mut head = ProtocolHead::default();
        let body = ProtocolBody::default();
        let body_size = body.size();
        head.set_data_size(body_size.try_into().unwrap());
        let p = Protocol {
            head: head,
            body: body,
        };
        p
    }
}

/// protocol header
#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ProtocolHead {
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
    pub fn size() -> usize {
        serialized_size(ProtocolHead::default())
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
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
pub struct ProtocolBody {
    pub message: MessageDTO,
}

impl ProtocolBody {
    pub fn new(message_dto: MessageDTO) -> Self {
        Self {
            message: message_dto,
        }
    }
    pub fn size(&self) -> usize {
        serialized_size(self)
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        decode(bytes)
    }
    pub fn to_byte_vec(&self) -> Vec<u8> {
        encode(self)
    }
    pub fn insert_message(&mut self, message_dto: MessageDTO) {
        self.message = message_dto;
    }
}

impl Default for ProtocolBody {
    fn default() -> Self {
        Self {
            message: MessageDTO::default(),
        }
    }
}
