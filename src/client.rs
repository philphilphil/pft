use std::{
    io::{self, stdin, Write},
    net::{Shutdown, SocketAddr, TcpStream},
    path::{Path, PathBuf},
};

use crate::{
    request::{Request, TransferType},
    response::{FileTransferError, Response},
};

pub fn start(address: &SocketAddr, otp: String, filename: PathBuf) -> io::Result<()> {
    match TcpStream::connect(address) {
        Ok(mut stream) => {
            println!("Connection successfull to {}", address);

            sent_announce_upload_request(&mut stream, &filename, otp)?;

            let response = Response::deserialize(&mut stream).unwrap();
            if let Some(e) = response.error {
                match e {
                    FileTransferError::InvalidPassword => {
                        println!("ERROR: Invalid password.");
                    }
                    FileTransferError::FileAlreadyExists => {
                        let user_input = get_users_transfer_type_choice();
                        let transfer_type = match user_input.chars().next().unwrap() {
                            'c' | 'C' => return Ok(()),
                            'r' | 'R' => TransferType::Replace,
                            'k' | 'K' => TransferType::KeepBoth,
                            _ => {
                                println!("ERROR: Invalid input.");
                                return Ok(());
                            }
                        };
                        sent_upload_file_request(&mut stream, &filename, transfer_type)?;
                    }
                }
            } else {
                sent_upload_file_request(&mut stream, &filename, TransferType::Normal)?;
            }
        }
        Err(e) => {
            println!("ERROR: Failed to connect to {}: {}", address, e);
        }
    }
    Ok(())
}

fn get_users_transfer_type_choice() -> String {
    println!("ERROR: File alredy exists.");
    print!("You can (r)eplace it, (c)ancel or (k)eep both: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Input error.");
    choice
}

fn sent_announce_upload_request(
    stream: &mut TcpStream,
    filename: &Path,
    otp: String,
) -> io::Result<()> {
    let announce_req = Request::AnnounceFileTransfer {
        filename: filename.as_os_str().to_str().unwrap().to_string(),
        otp,
    };

    announce_req.serialize(stream)?;
    stream.flush()?;

    Ok(())
}

fn sent_upload_file_request(
    stream: &mut TcpStream,
    filename: &Path,
    transfer_type: TransferType,
) -> io::Result<()> {
    let req = Request::UploadFile {
        filename: filename.as_os_str().to_str().unwrap().to_string(),
        transfer_type,
    };

    req.serialize(stream)?;
    stream.flush()?;
    stream.shutdown(Shutdown::Write)?;

    let response = Response::deserialize(stream)?;
    println!("{}", response.message);

    Ok(())
}
