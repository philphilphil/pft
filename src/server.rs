use std::{fs::File, io, net::TcpListener};

use crate::Request;

pub fn start() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for mut stream in listener.incoming().flatten() {
        let mut file = File::create("downloadedfiles.txt").unwrap();
        io::copy(&mut stream, &mut file).unwrap();
        // let request = Request::deserialize(&mut stream).unwrap();
        // match request {
        //     Request::Echo(msg) => println!("{}", msg),
        //     Request::Jumble { .. } => todo!(),
        // }
    }
}
