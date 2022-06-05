use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    path::Path,
};

// use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{
    request::Request,
    response::{FileTransferError, Response},
};

pub fn start(address: &SocketAddr) {
    // let cotp: String = thread_rng()
    //     .sample_iter(&Alphanumeric)
    //     .take(30)
    //     .map(char::from)
    //     .collect();
    let cotp = "abc".to_string();

    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => {
            println!("ERROR: Can't bind to address {}: {}", address, e);
            return;
        }
    };

    println!("Starting pft server v{}.", env!("CARGO_PKG_VERSION"));
    println!("Listening on {}", address);
    println!("One-time password: {}", cotp);
    for mut stream in listener.incoming().flatten() {
        loop {
            let mut peek_buf = [0; 10];
            if stream.peek(&mut peek_buf).unwrap() == 0 {
                break;
            }

            let request = Request::deserialize(&mut stream).unwrap();

            match request {
                Request::AnnounceFileTransfer { filename, otp } => {
                    if cotp != otp {
                        let resp = Response {
                            message: "".to_string(),
                            error: Some(FileTransferError::InvalidOneTimePassword),
                        };
                        resp.serialize(&mut stream).unwrap();
                        stream.flush().unwrap();
                        println!("ERROR: Invalid OTP from {}", address);
                        continue;
                    }

                    if Path::new(&filename).exists() {
                        let resp = Response {
                            message: String::new(),
                            error: Some(FileTransferError::FileAlreadyExists),
                        };
                        resp.serialize(&mut stream).unwrap();
                        stream.flush().unwrap();
                        println!("ERROR: File alraedy exist from {}", address);
                    } else {
                        let resp = Response {
                            message: String::new(),
                            error: None,
                        };
                        resp.serialize(&mut stream).unwrap();
                        stream.flush().unwrap();
                    }
                }
                Request::UploadFile { filename: _ } => {
                    let resp = Response {
                        message: "Successfully transfered.".to_string(),
                        error: None,
                    };
                    resp.serialize(&mut stream).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
    }
}
