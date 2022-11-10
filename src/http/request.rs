use std::convert::{TryFrom, From};
use std::error::Error;
use std::fmt::{Formatter, Result as FmtResult, Display, Debug};
use std::str::Utf8Error;
use std::str;

use super::method::Method;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method
}

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {
        unimplemented!()
    }
}

impl TryFrom<&[u8]> for Request {
   type Error = ParseError;
   
   fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {

    let result = str::from_utf8(buf)?;
    unimplemented!()
   }
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
