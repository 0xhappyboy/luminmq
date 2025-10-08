/// a server module for lumin message queues.
use std::io::{self};

use luminmq_core::{
    group::Groups,
    protocol::Protocol,
    types::{CONNECTION_POOL, CONNECTION_POOL_GROUP_BIND},
};
use mio::{
    Events, Interest, Poll, Registry, Token,
    event::Event,
    net::{TcpListener, TcpStream},
};
use tokio::stream;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LISTENER_PORT;

const SERVER_TOKEN: Token = Token(0);

pub struct LuminMQServer;

impl LuminMQServer {
    pub async fn start() -> std::io::Result<()> {
        // enable log
        tracing_subscriber::registry().with(fmt::layer()).init();
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
                        CONNECTION_POOL_GROUP_BIND
                            .lock()
                            .unwrap()
                            .insert(token, ("group-test".to_string(), "topic-test".to_string()));
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
        // Protocol::writer(connection);
        // let mut protocol = &mut Protocol::default();
        // let mut msg = Message::default();
        // msg.group_id = "group-test".to_string();
        // msg.topic = Topic::new("topic-test".to_string());
        // msg.consumer_type = ConsumerType::Send;
        // msg.msg_type = MessageType::Business;
        // protocol.insert_message(msg.to_messagedto());
        // protocol.ready();
        // let protocol_buf = protocol.to_byte_vec();
        // match connection.write(&protocol_buf) {
        //     Ok(n) if n < protocol_buf.len() => return Err(io::ErrorKind::WriteZero.into()),
        //     Ok(_) => registry.reregister(connection, event.token(), Interest::READABLE)?,
        //     Err(ref err) if would_block(err) => {}
        //     Err(ref err) if interrupted(err) => {
        //         return handle_connection_event(registry, connection, event);
        //     }
        //     Err(err) => {
        //         return Err(err);
        //     }
        // }
    }

    if event.is_readable() {
        Protocol::handle(&connection);
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
