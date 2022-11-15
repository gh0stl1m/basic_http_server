use crate::http::Request;

use std::net::TcpStream;
use std::{net::TcpListener, io::Read};
use std::convert::TryFrom;
pub struct Server {
    addr: String,
}

impl Server {

    pub fn new(addr: String) -> Self {

        Self { addr }
    }

    pub fn run(&self) {

        println!("Server Running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {

            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    read_stream(stream, buffer);
                }
                Err(e) => println!("Failed to establish a connection: {}", e)
            }
        }
    }
}

fn read_stream(mut stream: TcpStream, mut buffer: [u8; 1024]) {

    match stream.read(&mut buffer) {
        Ok(_) => {
            println!("Got request {}", String::from_utf8_lossy(&buffer));

            match Request::try_from(&buffer[..]) {
                Ok(req) => {
                    dbg!(req);
                },
                Err(e) => println!("Failed to parse the request")
            }
        }
        Err(e) => println!("Got error opening TCP connection {}", e),
    }
}
