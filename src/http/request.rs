use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

use super::method::{Method, MethodError};

pub struct Request<'buffer> {
    path: &'buffer str,
    query_string: Option<&'buffer str>,
    method: Method,
}

impl<'buffer> TryFrom<&'buffer [u8]> for Request<'buffer> {
    type Error = ParseError;

    fn try_from(buf: &'buffer [u8]) -> Result<Self, Self::Error> {
        let request = from_utf8(buf)?;

        let (method, request) = fetch_next_word(request)
            .ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = fetch_next_word(request)
            .ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = fetch_next_word(request)
            .ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(&path[i + 1..]);
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}

fn fetch_next_word(request: &str) -> Option<(&str, &str)> {
    for (idx, char) in request.chars().enumerate() {
        if char == ' ' || char == '\r' {
            return Some((&request[..idx], &request[idx + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid request",
            Self::InvalidEncoding => "Invalid encoding",
            Self::InvalidProtocol => "Invalid protocol",
            Self::InvalidMethod => "Invalid method",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // todo!()
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // todo!()
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
