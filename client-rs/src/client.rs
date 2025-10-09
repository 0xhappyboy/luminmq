use std::{io::Write, thread, time::Duration};

use luminmq_core::{
    msg::{ConsumerType, Message, MessageStatus, MessageType},
    protocol::Protocol,
    topic::Topic,
    types::ConsumerBinder,
};
use mio::{Events, Interest, Poll, Token, net::TcpStream};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

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
                            Protocol::handle(&stream, |mss| {
                                // test
                                thread::sleep(Duration::from_millis(3000));
                                if mss.status == MessageStatus::Success {
                                    match ConsumerBinder::get((
                                        mss.group_id.to_string(),
                                        mss.topic.name.to_string(),
                                    )) {
                                        Ok(fn_consumer) => match fn_consumer(mss) {
                                            Ok(_s) => {
                                                // consumption success
                                            }
                                            Err(_e) => {
                                                // consumption fail
                                            }
                                        },
                                        Err(_e) => (),
                                    };
                                } else if mss.status == MessageStatus::Fail {
                                    // error return code
                                }
                            });
                        }

                        if event.is_writable() {
                            // test protocol
                            let protocol = &mut Protocol::default();
                            let mut msg = Message::default();
                            msg.group_id = "group-test".to_string();
                            msg.topic = Topic::new("topic-test".to_string());
                            msg.consumer_type = ConsumerType::Pull;
                            msg.msg_type = MessageType::Business;
                            msg.data = "12312".to_string();
                            protocol.insert_message(msg.to_messagedto());
                            let _ = protocol.ready();
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
                    _token => {}
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
