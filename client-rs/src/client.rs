use std::io::Write;

use luminmq_core::{
    msg::{ConsumerType, Message, MessageType},
    protocol::Protocol,
    topic::Topic,
};
use mio::{Events, Interest, Poll, Token, net::TcpStream};

use crate::config::LISTENER_PORT;
use std::io::{self};

/// luminmq client module
pub struct LuminMQClient;
impl LuminMQClient {
    pub async fn start() -> std::io::Result<()> {
        let addr = LISTENER_PORT.lock().unwrap().parse().unwrap();
        let mut stream = TcpStream::connect(addr)?;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);
        poll.registry().register(
            &mut stream,
            Token(0),
            Interest::READABLE | Interest::WRITABLE,
        )?;
        loop {
            poll.poll(&mut events, None)?;
            for event in &events {
                match event.token() {
                    Token(0) => {
                        if event.is_readable() {
                            match Protocol::reader(&stream) {
                                Ok(p) => {
                                    // received protocol
                                }
                                Err(e) => println!("{:?}", e),
                            }
                        }

                        if event.is_writable() {
                            let mut protocol = &mut Protocol::default();
                            let mut msg = Message::default();
                            msg.group_id = "group-test".to_string();
                            msg.topic = Topic::new("topic-test".to_string());
                            msg.consumer_type = ConsumerType::Send;
                            msg.msg_type = MessageType::Business;
                            protocol.insert_message(msg.to_messagedto());
                            protocol.ready();
                            let protocol_buf = protocol.to_byte_vec();
                            match stream.write(&protocol_buf) {
                                Ok(n) if n < protocol_buf.len() => {
                                    return Err(io::ErrorKind::WriteZero.into());
                                }
                                Ok(_) => {}
                                Err(ref err) if would_block(err) => {}
                                Err(ref err) if interrupted(err) => {}
                                Err(err) => {
                                    return Err(err);
                                }
                            }
                        }
                    }
                    token => {}
                    _ => (),
                }
            }
        }
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
