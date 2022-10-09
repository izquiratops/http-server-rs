use crate::http::{ParseError, Request, Response, StatusCode};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response;
}

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, _: &Request) -> Response {
        Response::new(
            StatusCode::Ok,
            Some("<h1>Testing with handlers</h1>".to_string())
        )
    }

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("failed to parse request {e:?}");
        Response::new(
            StatusCode::BadRequest,
            None
        )
    }
}
