use crate::Request;
use std::io::Read;
use std::net::TcpListener;

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
                            Ok(req) => println!("accepting request: {req:#?}"),
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
