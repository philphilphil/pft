use std::{
    fs::File,
    io,
    net::{SocketAddr, TcpListener},
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn start(address: &SocketAddr) {
    let otp: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let listener = TcpListener::bind(address).unwrap();
    println!("Starting pft server v{}.", env!("CARGO_PKG_VERSION"));
    println!("Listening on {}", address);
    println!("One-time password: {}", otp);

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
