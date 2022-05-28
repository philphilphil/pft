use std::net::TcpStream;

use crate::LinesCodec;

pub fn start() {
    match TcpStream::connect("localhost:3030") {
        Ok(stream) => {
            println!("Connection successfull on port 3030");
            let mut codec = LinesCodec::new(stream).unwrap();
            codec.send_message("does this still work?").unwrap();

            println!("{}", codec.read_message().unwrap());
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
