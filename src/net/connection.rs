use std::io::Read;

use bytes::BytesMut;
use mio::net::TcpStream;
use crate::command::Parser;
use redis_protocol::resp3::types::OwnedFrame;

pub struct Connection {
    pub stream:     TcpStream,
    pub read_buf:   [u8; 8 * 1024],          // raw bytes from socket
    pub parser:     Parser,            // keeps decode state + remainder
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            read_buf: [0; 8 * 1024],
            parser:   Parser::new(),
        }
    }

    pub fn read_and_parse(&mut self) -> Result<Option<Vec<OwnedFrame>>, Box<dyn std::error::Error>> {
        let mut commands: Vec<OwnedFrame> = Vec::new();
        loop {
            match self.stream.read(&mut self.read_buf) {
                Ok(0) => {
                    return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Connection closed")));
                }
                Ok(bytes_read) => {
                    match self.parser.parse_frame(&self.read_buf[..bytes_read])? {
                        Some(new_commands) => {
                            commands.extend(new_commands);
                        }
                        None => {
                            return Ok(Some(commands));
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    return Ok(None);
                }
                Err(e) => {
                    println!("Read error: {:?}", e);
                    return Err(Box::new(e));
                }
            }
        }
    }
}