use crate::http::{Request, Response, StatusCode, ParseError};

use std::net::TcpStream;
use std::{net::TcpListener, io::Read};
use std::convert::TryFrom;

pub trait Handler {

    fn handle_request(&mut self, request: &Request)  -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);

        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {

    pub fn new(addr: String) -> Self {

        Self { addr }
    }

    pub fn run(&self, handler: impl Handler + Copy) {

        println!("Server Running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept() {

                Ok((stream, _)) => {
                    let buffer = [0; 1024];

                    handle_connection(stream, buffer, handler);
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, mut buffer: [u8; 1024], mut handler: impl Handler) {

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Got request {}", String::from_utf8_lossy(&buffer));

            let response = match Request::try_from(&buffer[..]) {
                Ok(req) => handler.handle_request(&req),
                Err(e) => handler.handle_bad_request(&e),
            };

            if let Err(err) = response.send(&mut stream) {
                println!("Failed to send response to the TCP stream: {}", err);
            }

        }
        Err(e) => println!("Got error opening TCP connection {}", e),
    }
}
