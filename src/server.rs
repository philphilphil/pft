use std::net::TcpListener;

use crate::LinesCodec;

pub fn start() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for stream in listener.incoming().flatten() {
        let mut codec = LinesCodec::new(stream).unwrap();

        let message: String = codec
            .read_message()
            .map(|m| m.chars().rev().collect())
            .unwrap();

        codec.send_message(&message).unwrap();
    }
}
