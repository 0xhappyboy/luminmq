/// a server module for lumin message queues.
use std::io::{self};

use luminmq_core::{
    group::Groups,
    msg,
    protocol::Protocol,
    types::{ConnectionPool, ConnectionPoolAndGroupBind},
};
use mio::{
    Events, Interest, Poll, Registry, Token,
    event::Event,
    net::{TcpListener, TcpStream},
};
use tokio::stream;
use tracing::{Level, event, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LISTENER_PORT;

const SERVER_TOKEN: Token = Token(0);

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
        event!(Level::INFO, "STARTED SUCCESS");
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
                        ConnectionPool::insert(token, connection);
                        ConnectionPoolAndGroupBind::insert(
                            token,
                            ("group-test".to_string(), "topic-test".to_string()),
                        );
                    },
                    // system buffer changes
                    token => {
                        ConnectionPool::handle(&token, |stream| {
                            event!(
                                Level::INFO,
                                "client access received, address:{:?}",
                                stream.peer_addr()
                            );
                            let flag = match handle_connection_event(stream, event) {
                                Ok(_) => false,
                                Err(_) => true,
                            };
                            if flag {
                                ConnectionPool::remove(token);
                            }
                        });
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_connection_event(connection: &mut TcpStream, event: &Event) -> io::Result<bool> {
    if event.is_readable() {
        Protocol::handle(&connection, |msg| {});
    }
    Ok(false)
}

fn next(current: &mut Token) -> Token {
    let next = current.0;
    current.0 += 1;
    Token(next)
}
