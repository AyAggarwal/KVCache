use std::collections::HashMap;
use std::io::{Read, Write};
use std::time::Duration;
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::net::SocketAddr;
use bytes::{BufMut, Bytes, BytesMut};

use crate::net::Connection;


pub struct Server {
}

impl Server {
    pub fn new() -> Self {
        Self {
         //TODO: config
        }
    }

    pub fn start(&self) -> std::io::Result<()> {
        
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);

        let server_socket_addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
        let mut server_listener = TcpListener::bind(server_socket_addr)?;


        const SERVER: Token = Token(10000);
        let mut connections: HashMap<Token,Connection> = HashMap::new();
        
        poll.registry().register(&mut server_listener, SERVER, Interest::READABLE)?;

        let mut next_token = 0;

        loop {
            poll.poll(&mut events, Some(Duration::from_millis(100)))?;
            for event in events.iter() {
                match event.token() {
                    SERVER => loop {
                        match server_listener.accept() {
                            Ok((mut connection, _)) => {
                                
                                let token = Token(next_token); // TODO: Max Connections
                                next_token += 1;

                                poll.registry().register(
                                    &mut connection,
                                    token,
                                    Interest::READABLE
                                )?;

                                let conn = Connection::new(connection);

                                connections.insert(token, conn); 
                            },
                            Err(ref err) if would_block(err) => break,
                            Err(err) => {
                                return Err(err);
                            },
                        }
                    },
                    token => {
                        let mut should_remove = false;
                        match connections.get_mut(&token) {
                            Some(conn) => {
                                match conn.read_and_parse() {
                                    Ok(Some(commands)) => {
                                        println!("Received {} commands: {:?}", commands.len(), commands);
                                        // TODO: Handle commands
                                    },
                                    Ok(None) => {},
                                    Err(e) => {
                                        should_remove = true;
                                        println!("Connection closed: {:?}", e);
                                    }
                                }
                            },
                            None => {
                                println!("Unexpected error getting connection");
                                should_remove = true;
                            },
                        }     
                        if should_remove {
                            if let Some(mut conn) = connections.remove(&token) {
                                poll.registry().deregister(&mut conn.stream)?;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn would_block(err: &std::io::Error) -> bool {
    err.kind() == std::io::ErrorKind::WouldBlock
}