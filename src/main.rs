fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server {
            addr: addr.to_string()
        }
    }

    fn run(&self) {
        println!("Listening on {}!", self.addr);
    }
}