/// communication protocol
use bincode::{Decode, Encode, error::DecodeError};
use mio::net::TcpStream;
use std::io::{BufRead, BufReader, Write};
use tracing::{Level, event};

use crate::{
    msg::{Message, MessageDTO},
    tool::codec::{decode, encode, serialized_size},
};

// fixed protocol identifier
pub const PROTOCOL_IDENTIFIER: &str = "luminmq";
// Fixed protocol end identifier
pub const PROTOCOL_END_IDENTIFIER: &str = "END";

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct Protocol {
    pub head: ProtocolHead,
    pub body: ProtocolBody,
    pub end: ProtocolEnd,
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
        self.end = ProtocolEnd::default();
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
    // protocol handle
    pub fn handle(stream: &TcpStream) {
        let mut r: BufReader<&TcpStream> = BufReader::new(stream);
        loop {
            match Protocol::reader(&mut r) {
                Ok(protocol) => {
                    // protocol
                }
                Err(e) => {
                    break;
                }
            }
        }
    }
    // protocol reader
    pub fn reader(r: &mut BufReader<&TcpStream>) -> Result<Protocol, String> {
        let mut protocol = Protocol::default();
        loop {
            match ProtocolHead::reader(r) {
                Ok(h) => match ProtocolBody::reader(r, h.data_size.try_into().unwrap()) {
                    Ok(body) => match ProtocolEnd::reader(r) {
                        Ok(end) => {
                            protocol.head = h;
                            protocol.body = body;
                            protocol.end = end;
                            return Ok(protocol);
                        }
                        Err(e) => return Err(e),
                    },
                    Err(e) => return Err(e),
                },
                Err(e) => {
                    return Err(e);
                }
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
            end: ProtocolEnd::default(),
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
    // build protocol head
    pub fn build(bytes: &[u8]) -> Result<Self, String> {
        match ProtocolHead::from_bytes(bytes) {
            Ok(head) => {
                return Ok(head);
            }
            Err(_) => return Err("Protocol header serialization exception".to_string()),
        }
    }
    // protocol head reader
    pub fn reader(r: &mut BufReader<&TcpStream>) -> Result<ProtocolHead, String> {
        match r.fill_buf() {
            Ok(buf) => {
                let head_size = Protocol::protocol_head_size();
                let buf_len = buf.len();
                if buf_len >= head_size {
                    let head_buf = &buf[..head_size];
                    if Protocol::verify_protocol_head(&head_buf) {
                        // build protocol head
                        match ProtocolHead::build(&head_buf) {
                            Ok(head) => {
                                r.consume(head_size);
                                return Ok(head);
                            }
                            Err(e) => {
                                r.consume(head_size);
                                return Err(e);
                            }
                        }
                    } else {
                        r.consume(head_size);
                        return Err(
                            "The protocol header does not meet the requirements".to_string()
                        );
                    }
                } else if buf_len < head_size {
                    r.consume(buf_len);
                    event!(
                        Level::WARN,
                        "Dirty data, the buffer data does not meet the protocol header size."
                    );
                    return Err(
                        "Dirty data, the buffer data does not meet the protocol header size."
                            .to_string(),
                    );
                } else if buf_len == 0 {
                    event!(Level::WARN, "Connection is closed.");
                    return Err("Connection is closed.".to_string());
                } else {
                    return Err("Other exceptions".to_string());
                }
            }
            Err(_) => {
                return Err("Not enough data in buffer".to_string());
            }
        }
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
    // build protocol body
    pub fn build(bytes: &[u8]) -> Result<ProtocolBody, String> {
        match ProtocolBody::from_bytes(bytes) {
            Ok(body) => {
                return Ok(body);
            }
            Err(_e) => return Err("Protocol body serialization exception".to_string()),
        }
    }
    // protocol head reader
    pub fn reader(r: &mut BufReader<&TcpStream>, size: usize) -> Result<ProtocolBody, String> {
        match r.fill_buf() {
            Ok(buf) => {
                let buf_len = buf.len();
                if buf_len >= size {
                    let body_buf = &buf[..size];
                    match ProtocolBody::build(body_buf) {
                        Ok(body) => {
                            r.consume(size);
                            return Ok(body);
                        }
                        Err(e) => {
                            r.consume(size);
                            return Err(e);
                        }
                    }
                } else {
                    r.consume(buf_len);
                    return Err("Dirty data.".to_string());
                }
            }
            Err(_) => {
                return Err("Not enough data in buffer".to_string());
            }
        }
    }
}

impl Default for ProtocolBody {
    fn default() -> Self {
        Self {
            message: MessageDTO::default(),
        }
    }
}

#[derive(Encode, Decode, PartialEq, Debug, Clone)]
pub struct ProtocolEnd {
    flag: String,
}

impl ProtocolEnd {
    pub fn new() -> Self {
        Self {
            flag: PROTOCOL_END_IDENTIFIER.to_string(),
        }
    }
    // protocol end reader
    pub fn reader(r: &mut BufReader<&TcpStream>) -> Result<ProtocolEnd, String> {
        match r.fill_buf() {
            Ok(buf) => {
                let buf_len = buf.len();
                let end_len = serialized_size(PROTOCOL_END_IDENTIFIER);
                if buf_len >= end_len {
                    let end_identifier_buf = &buf[..end_len];
                    match decode::<String>(end_identifier_buf) {
                        Ok(end_identifier) => {
                            if PROTOCOL_END_IDENTIFIER
                                .to_string()
                                .eq(&end_identifier.to_string())
                            {
                                r.consume(end_len);
                                return Ok(ProtocolEnd::new());
                            } else {
                                r.consume(end_len);
                                return Err(
                                    "The ending mark does not meet the requirements".to_string()
                                );
                            }
                        }
                        Err(e) => {
                            r.consume(end_len);
                            return Err(format!("End identifier serialization exception: {:?}", e)
                                .to_string());
                        }
                    }
                } else {
                    r.consume(end_len);
                    return Err("dirty data.".to_string());
                }
            }
            Err(_) => {
                return Err("Not enough data in buffer".to_string());
            }
        }
    }
}

impl Default for ProtocolEnd {
    fn default() -> Self {
        Self {
            flag: PROTOCOL_END_IDENTIFIER.to_string(),
        }
    }
}
