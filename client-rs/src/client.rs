use std::io::{BufReader, Write};

use mio::{Events, Interest, Poll, Token, net::TcpStream};

use crate::{config::LISTENER_PORT, protocol::Protocol};
use std::io::{self, BufRead, BufWriter, Read};

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
                            let mut r = BufReader::new(&stream);
                            let mut protocol_head_buf = vec![0u8; Protocol::protocol_head_size()];
                            match r.read(&mut protocol_head_buf) {
                                Ok(0) => {
                                    // connection close
                                }
                                Ok(n) => {
                                    if Protocol::verify_protocol_head(&protocol_head_buf) {
                                        // build protocol
                                        let mut protocol = &mut Protocol::default();
                                        // build protocol head
                                        protocol = protocol
                                            .build_protocol_head_by_bytes(&protocol_head_buf);
                                        // consume system buffer
                                        r.consume(Protocol::protocol_head_size());
                                        // data area size
                                        let data_area_size = protocol.protocol_body_size();
                                        if data_area_size > 0 {
                                            let mut protocol_body_buf =
                                                vec![0u8; data_area_size.try_into().unwrap()];
                                            // data area buf
                                            r.read(&mut protocol_body_buf);
                                            protocol
                                                .build_protocol_body_by_bytes(&protocol_body_buf);
                                        }
                                    } else {
                                        // discard invalid data areas
                                        r.consume(Protocol::protocol_head_size());
                                    }
                                }
                                Err(ref err) if would_block(err) => {}
                                Err(ref err) if interrupted(err) => {}
                                Err(err) => return Err(err),
                            }
                        }

                        if event.is_writable() {
                            let mut protocol = &mut Protocol::default();
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
        Ok(())
    }
}

fn would_block(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn interrupted(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Interrupted
}
