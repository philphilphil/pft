use std::{
    io::Write,
    net::{Shutdown, SocketAddr, TcpStream},
    path::PathBuf,
};

use crate::{request::Request, response::Response};

pub fn start(address: &SocketAddr, otp: String, filename: PathBuf) {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connection successfull to {}", address);

            let announce_req = Request::AnnounceFileTransfer {
                filename: filename.as_os_str().to_str().unwrap().to_string(),
                otp,
            };

            announce_req.serialize(&mut stream).unwrap();
            stream.flush().unwrap();

            let response = Response::deserialize(&mut stream).unwrap();

            if let Some(e) = response.error {
                match e {
                    crate::response::FileTransferError::InvalidOneTimePassword => {
                        println!("ERROR: Invalid password.");
                        return;
                    }
                    crate::response::FileTransferError::FileAlreadyExists => {
                        println!("ERROR: File alredy exists.");
                        return;
                    }
                }
            } else {
                let req = Request::UploadFile {
                    filename: filename.as_os_str().to_str().unwrap().to_string(),
                };

                req.serialize(&mut stream).unwrap();
                stream.flush().unwrap();
                stream.shutdown(Shutdown::Write).unwrap();

                let response = Response::deserialize(&mut stream).unwrap();
                println!("{}", response.message);
            }
        }
        Err(e) => {
            println!("ERROR: Failed to connect to {}: {}", address, e);
        }
    }
}
