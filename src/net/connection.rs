use mio::net::TcpStream;
use crate::command::Parser;

pub struct Connection {
    pub stream: TcpStream,
    pub buffer: [u8; 1024],
    parser: Parser,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: [0; 1024],
            parser: Parser::new(),
        }
    }
}


