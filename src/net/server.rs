use std::collections::HashMap;
use std::io::{Read, Write};
use std::time::Duration;
use mio::{Events, Interest, Poll, Token};
use mio::net::TcpListener;
use std::net::SocketAddr;

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
                                    Interest::READABLE | Interest::WRITABLE,
                                )?;

                                let conn = Connection::new(connection);

                                connections.insert(token, conn); 
                                break;
                            },
                            Err(ref err) if would_block(err) => {
                                println!("Would block error: {}", err);
                                break;
                            },
                            Err(err) => {
                                eprintln!("Error accepting connection: {}", err);
                                return Err(err);
                            },
                        }
                    },
                    token => {
                        loop {
                            match connections.get_mut(&token) {
                                Some(conn) => {
                                    match conn.stream.read(&mut conn.buffer) {
                                        Ok(0) => {
                                            connections.remove(&token);
                                            break;
                                        }
                                        Ok(bytes_read) => {
                                            println!("Received {} bytes", bytes_read);
                                             
                                            conn.stream.write_all(&conn.buffer)?; // echo back   
        
                                            break;
                                        }
                                        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                            break;
                                        }
                                        e => panic!("err={:?}", e),
                                    }
                                },
                                None => {
                                    println!("Unexpected error getting connection");
                                    break
                                },
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
