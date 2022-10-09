#![allow(dead_code)]

use crate::server::Server;
use crate::website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    let website_handler = WebsiteHandler;

    server.run(website_handler);
}
