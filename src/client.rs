use std::{
    fs::File,
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
};

use crate::request::Request;

pub fn start(address: &SocketAddr) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connection successfull to {}", address);
            let mut buffer = String::new();
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer).unwrap();
            buffer.truncate(30); // remmove new line
                                 //
            let req = Request::TestOTP(buffer);
            let _serialized = req.serialize(&mut stream);
            stream.flush().unwrap();

            //             let mut file = File::open("testfile.txt").unwrap();

            //             let mut buf = [0; 4096];
            //             loop {
            //                 let n = file.read(&mut buf).unwrap();

            //                 if n == 0 {
            //                     // reached end of file
            //                     break;
            //                 }

            //                 stream.write_all(&buf[..n]).unwrap();
            // }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
