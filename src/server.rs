use std::{
    io::{self, BufRead},
    net::TcpListener,
};

pub fn start() {
    let listener = TcpListener::bind("localhost:3030").unwrap();

    for mut stream in listener.incoming().flatten() {
        let mut reader = io::BufReader::new(&mut stream);

        let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();

        reader.consume(received.len());

        String::from_utf8(received)
            .map(|s| println!("{}", s))
            .map_err(|_| println!("Error parsing received string as uff8"))
            .unwrap();
    }
}
