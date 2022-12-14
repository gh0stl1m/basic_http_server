use std::convert::{TryFrom, From};
use std::error::Error;
use std::fmt::{Formatter, Result as FmtResult, Display, Debug};
use std::str::Utf8Error;
use std::str;

use super::method::{Method, MethodError};
use super::QueryString;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method
}

impl<'buf> Request<'buf> {
    fn from_byte_array(buf: &'buf [u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
   type Error = ParseError;
   
   fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {

        let result = str::from_utf8(buf)?;
        let (method, request) = get_next_word(result).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        if let Some(path_index) = path.find('?') {

            path = &path[..path_index];
            query_string = Some(QueryString::from(&path[path_index + 1 ..]));
        }

        Ok(Self { path, query_string, method })
   }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {

    for(i, c) in request.chars().enumerate() {

        if c == ' ' || c == '\r' {

            return Some((&request[..i], &request[i + 1..]));
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
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEnconding",
            Self::InvalidProtocol => "InvalidProtocol",
            Self::InvalidMethod =>"InvalidMethod",
        }
    }
    
}

impl From<MethodError> for ParseError {

    fn from(error: MethodError) -> Self {
        Self::InvalidMethod
    }
    
}

impl From<Utf8Error> for ParseError {

    fn from(error: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
    
}

impl Display for ParseError {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
