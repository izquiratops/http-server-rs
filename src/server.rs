use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr: addr.to_string()
        }
    }

    pub fn run(&self) {
        println!("Listening on {}!", &self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((_socket, addr)) => println!("new client: {addr:?}"),
                Err(e) => println!("couldn't get client: {e:?}"),
            }
        }
    }
}