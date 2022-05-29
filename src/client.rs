use std::{io::Write, net::TcpStream};

use crate::Request;

pub fn start() {
    match TcpStream::connect("localhost:3030") {
        Ok(mut stream) => {
            println!("Connection successfull on port 3030");

            let req = Request::Echo("does this really still work?".to_string());
            let _serialized = req.serialize(&mut stream);
            stream.flush().unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
