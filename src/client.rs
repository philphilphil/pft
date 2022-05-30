use std::{
    fs::File,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
};

pub fn start(address: &SocketAddr) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connection successfull on port 3030");

            // let req = Request::Echo("does this really still work?".to_string());
            // let _serialized = req.serialize(&mut stream);
            // stream.flush().unwrap();
            let mut file = File::open("testfile.txt").unwrap();

            let mut buf = [0; 4096];
            loop {
                let n = file.read(&mut buf).unwrap();

                if n == 0 {
                    // reached end of file
                    break;
                }

                stream.write_all(&buf[..n]).unwrap();
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
