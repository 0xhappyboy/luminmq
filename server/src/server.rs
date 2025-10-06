/// a server module for lumin message queues.
use std::{
    collections::HashMap,
    io::{self, Write},
    sync::Mutex,
};

use luminmq_core::{
    msg::{ConsumerType, Message, MessageType},
    protocol::Protocol,
    topic::Topic,
};
use mio::{
    Events, Interest, Poll, Registry, Token,
    event::Event,
    net::{TcpListener, TcpStream},
};

use crate::config::LISTENER_PORT;
use lazy_static::lazy_static;

const SERVER_TOKEN: Token = Token(0);

lazy_static! {
    // connection pool
    pub static ref CONNECTION_POOL: Mutex<HashMap<Token, TcpStream>> = Mutex::new(HashMap::<Token, TcpStream>::default());
    // connection pool and gourp bind
    // k: token v: (group id, topic)
    pub static ref CONNECTION_POOL_GROUP_BIND: Mutex<HashMap<Token, (String, String)>> = Mutex::new(HashMap::<Token, (String, String)>::default());
}

pub struct LuminMQServer;

impl LuminMQServer {
    pub async fn start() -> std::io::Result<()> {
        let addr = LISTENER_PORT.lock().unwrap().parse().unwrap();
        let mut listener = TcpListener::bind(addr)?;
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(1024);
        // let mut connections = HashMap::<Token, TcpStream>::new();
        poll.registry()
            .register(&mut listener, Token(0), Interest::READABLE)
            .unwrap();
        let mut unique_token = Token(SERVER_TOKEN.0 + 1);

        loop {
            poll.poll(&mut events, None).unwrap();
            for event in &events {
                match event.token() {
                    SERVER_TOKEN => loop {
                        let (mut connection, _address) = match listener.accept() {
                            Ok((connection, address)) => (connection, address),
                            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                break;
                            }
                            Err(_e) => {
                                break;
                            }
                        };
                        let token = next(&mut unique_token);
                        poll.registry().register(
                            &mut connection,
                            token,
                            Interest::READABLE.add(Interest::WRITABLE),
                        )?;
                        CONNECTION_POOL.lock().unwrap().insert(token, connection);
                    },
                    // system buffer changes
                    token => {
                        let done = if let Some(connection) =
                            CONNECTION_POOL.lock().unwrap().get_mut(&token)
                        {
                            match handle_connection_event(poll.registry(), connection, event) {
                                Ok(_) => false,
                                Err(_) => true,
                            }
                        } else {
                            false
                        };
                        if done {
                            CONNECTION_POOL.lock().unwrap().remove(&token);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_connection_event(
    registry: &Registry,
    connection: &mut TcpStream,
    event: &Event,
) -> io::Result<bool> {
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
        match connection.write(&protocol_buf) {
            Ok(n) if n < protocol_buf.len() => return Err(io::ErrorKind::WriteZero.into()),
            Ok(_) => registry.reregister(connection, event.token(), Interest::READABLE)?,
            Err(ref err) if would_block(err) => {}
            Err(ref err) if interrupted(err) => {
                return handle_connection_event(registry, connection, event);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }
    if event.is_readable() {
        match Protocol::reader(&connection) {
            Ok(p) => {}
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(false)
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
