use std::{
    fs::File,
    io::{self, Read, Write},
    net::{Shutdown, SocketAddr, TcpStream},
};

use crate::{request::Request, response::Response};

pub fn start(address: &SocketAddr) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connection successfull to {}", address);
            //let mut buffer = String::new();
            //let stdin = io::stdin(); // We get `Stdin` here.
            //stdin.read_line(&mut buffer).unwrap();
            //buffer.truncate(30); // remmove new line
            //                     //
            //let req = Request::TestOTP(buffer);

            let req = Request::UploadFile {
                filename: "testfile.txt".to_string(),
            };

            let _serialized = req.serialize(&mut stream);
            stream.flush().unwrap();
            stream.shutdown(Shutdown::Write).unwrap();

            let response = Response::deserialize(&mut stream).unwrap();
            println!("{}", response.0);
        }
        Err(e) => {
            println!("ERROR: Failed to connect to {}: {}", address, e);
        }
    }
}
