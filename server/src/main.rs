use lazy_static::lazy_static;
use luminmq_server::config::LISTENER_PORT;
use luminmq_server::server::LuminMQServer;
use mio::event::Event;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Registry, Token};
use std::collections::HashMap;
use std::f32::consts::E;
use std::io::{self, Read, Write};
use std::mem;
use std::net::SocketAddr;
use std::str::from_utf8;

const SERVER_TOKEN: Token = Token(0);
const DATA: &[u8] = b"Hello world!\n";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LuminMQServer::start().await;
    Ok(())
}
