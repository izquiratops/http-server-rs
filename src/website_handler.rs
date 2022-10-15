use std::fs;

use crate::http::{Method, ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response;
}

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        fs::read_to_string(path).ok()
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::Ok,
                    self.read_file("index.html")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(
                        StatusCode::Ok,
                        Some(contents)),
                    None => Response::new(
                        StatusCode::NotFound,
                        None)
                }
            }
            _ => Response::new(
                StatusCode::NotFound,
                None)
        }
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("failed to parse request {e:?}");
        Response::new(
            StatusCode::BadRequest,
            None,
        )
    }
}
