use std::{
    io::{self, Write},
    net::{SocketAddr, TcpListener},
    path::Path,
};

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::{
    request::Request,
    response::{FileTransferError, Response},
};

pub fn start(address: &SocketAddr) -> io::Result<()> {
    let generated_pw: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    // let cotp = "abc".to_string();

    let listener = match TcpListener::bind(address) {
        Ok(l) => l,
        Err(e) => {
            println!("ERROR. Can't bind to address {}: {}", address, e);
            return Ok(());
        }
    };

    println!("Starting pft server v{}.", env!("CARGO_PKG_VERSION"));
    println!("Listening on {}", address);
    println!("One-time password: {}", generated_pw);
    for mut stream in listener.incoming().flatten() {
        let client_addr = stream.peer_addr().unwrap().to_string();
        loop {
            let mut peek_buf = [0; 10];
            if stream.peek(&mut peek_buf).unwrap() == 0 {
                break;
            }

            let request = Request::deserialize(&mut stream, &client_addr).unwrap();
            match request {
                Request::AnnounceFileTransfer { filename, otp } => {
                    if generated_pw != otp {
                        let resp = Response {
                            message: "".to_string(),
                            error: Some(FileTransferError::InvalidPassword),
                        };
                        resp.serialize(&mut stream).unwrap();
                        stream.flush().unwrap();
                        println!("ERROR [{}]: Invalid OTP received.", client_addr);
                        continue;
                    }

                    if Path::new(&filename).exists() {
                        let resp = Response {
                            message: String::new(),
                            error: Some(FileTransferError::FileAlreadyExists),
                        };
                        resp.serialize(&mut stream).unwrap();
                        stream.flush().unwrap();
                        println!(
                            r#"ERROR [{}]: File "{}" alraedy exist."#,
                            client_addr, filename
                        );
                    } else {
                        let resp = Response {
                            message: String::new(),
                            error: None,
                        };
                        resp.serialize(&mut stream).unwrap();
                        stream.flush().unwrap();
                    }
                }
                Request::UploadFile {
                    transfer_type: _,
                    filename,
                } => {
                    let resp = Response {
                        message: format!(r#"Successfully transfered "{}""#, filename),
                        error: None,
                    };
                    resp.serialize(&mut stream).unwrap();
                    stream.flush().unwrap();
                }
            }
        }
    }
    Ok(())
}
