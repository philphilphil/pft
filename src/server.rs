use std::net::TcpListener;

use crate::{Request, Response};

pub fn start() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for mut stream in listener.incoming().flatten() {
        let request = Request::deserialize(&mut stream).unwrap();
        match request {
            Request::Echo(msg) => println!("{}", msg),
            Request::Jumble { message, amount } => todo!(),
        }
    }
}
