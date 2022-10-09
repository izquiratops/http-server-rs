use std::io::Read;
use std::net::TcpListener;

use crate::http::Request;
use crate::website_handler::Handler;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr: addr.to_string(),
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}!", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            let response = match Request::try_from(&buffer[..]) {
                                Ok(req) => handler.handle_request(&req),
                                Err(e) => handler.handle_bad_request(&e)
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("failed to send response: {e:?}");
                            }
                        }
                        Err(e) => println!("failed to read from connection: {e:?}")
                    }
                }
                Err(e) => println!("couldn't get client: {e:?}")
            }
        }
    }
}
