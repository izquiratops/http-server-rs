#![allow(dead_code)]

use std::env;
use crate::server::Server;
use crate::website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("Setting public directory on: {public_path:?}");

    server.run(WebsiteHandler::new(public_path));
}
