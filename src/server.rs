use std::io::{Read};
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
            self.get_new_connection(&listener);
        }
    }

    fn get_new_connection(&self, listener: &TcpListener) {
        match listener.accept() {
            Ok((mut stream, addr)) => {
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(req) => {
                        let response = match Request::try_from(&buffer[..]) {
                            Ok(req) => {
                                Response::new(
                                    StatusCode::Ok,
                                    Some("<h1>Hello wordl!</h1>".to_string()),
                                )
                            }
                            Err(e) => {
                                Response::new(
                                    StatusCode::BadRequest,
                                    None)
                            }
                        };

                        if let Err(e) = response.send(&mut stream) {
                            println!("failed to send response: {e:?}");
                        }
                    }
                    Err(e) => {
                        println!("failed to read from connection: {e:?}");
                    }
                }
            }
            Err(e) => {
                println!("couldn't get client: {e:?}");
            }
        }
    }
}
