use luminmq_client::client::LuminMQClient;
use luminmq_client::protocol::Protocol;
use mio::net::TcpStream;
use mio::{Events, Interest, Poll, Token};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::SocketAddr;
use std::os::windows::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    LuminMQClient::start().await;
    Ok(())
}
