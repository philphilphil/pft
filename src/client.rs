use std::{io::Write, net::TcpStream};

pub fn start() {
    match TcpStream::connect("localhost:3030") {
        Ok(mut stream) => {
            println!("Connection successfull on port 3030");
            stream.write_all(b"does this work?").unwrap();
            stream.flush().unwrap();
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
