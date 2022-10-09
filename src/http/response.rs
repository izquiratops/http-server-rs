use std::io::{Result as IoResult, Write};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    /*  'impl Write' defines a new func for every different type called on runtime!
     *  So we end up with a lot of implementations of the same function,
     *  but each one expecting a different argument.
     */
    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> { // <()> means no data
        let body_content = match &self.body {
            Some(b) => b,
            None => ""
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n {}",
            self.status_code,
            self.status_code.reason_phrase(),
            body_content
        )
    }
}
