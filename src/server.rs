use std::net::{SocketAddr, TcpListener};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{request::Request, response::Response};

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
        let request = Request::deserialize(&mut stream).unwrap();
        match request {
            Request::TestOTP(msg) => {
                if msg == otp {
                    println!(
                        "Successfull OTP test attempt from {}",
                        stream.peer_addr().unwrap()
                    );
                    let resp = Response("asd".to_string());
                    resp.serialize(&mut stream).unwrap();
                } else {
                    println!(
                        "Failed OTP test attempt from {}",
                        stream.peer_addr().unwrap()
                    );
                }
            }
            Request::UploadFile { filename: _ } => {
                let resp = Response("Successfully transfered.".to_string());
                resp.serialize(&mut stream).unwrap();
            }
        }
    }
}
