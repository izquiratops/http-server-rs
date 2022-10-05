use std::io::{Read, Write};
use std::net::TcpListener;

use crate::http::{Response, StatusCode};
use crate::Request;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr: addr.to_string(),
        }
    }

    pub fn run(&self) {
        println!("Listening on {}!", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("new client: {addr:?}");

                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => match Request::try_from(&buffer[..]) {
                            Ok(req) => {
                                println!("accepting request: {req:#?}");
                                let response = Response::new(
                                    StatusCode::Ok,
                                    Some("<h1>Hello wordl!</h1>".to_string())
                                );
                                write!(stream, "{}", response).expect("TODO: panic message");
                            }
                            Err(e) => println!("failed to parse request: {e:?}"),
                        },
                        Err(e) => {
                            println!("failed to read connection: {e:?}");
                        }
                    }
                }
                Err(e) => {
                    println!("couldn't get client: {e:?}");
                }
            }
        }
    }
}
