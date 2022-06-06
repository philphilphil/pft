use crate::{extract_string, write_string};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{self, Read, Write},
};

pub enum Request {
    /// Announce file transfer to server
    AnnounceFileTransfer { filename: String, otp: String },
    /// Upload File
    UploadFile {
        transfer_type: TransferType,
        filename: String,
    },
}

pub enum TransferType {
    Normal,
    Replace,
    KeepBoth,
}

impl From<&TransferType> for u8 {
    fn from(error: &TransferType) -> Self {
        match error {
            TransferType::Normal => 1,
            TransferType::Replace => 2,
            TransferType::KeepBoth => 3,
        }
    }
}

impl From<u8> for TransferType {
    fn from(e: u8) -> Self {
        match e {
            1 => TransferType::Normal,
            2 => TransferType::Replace,
            3 => TransferType::KeepBoth,
            _ => panic!("Error deserializing."),
        }
    }
}

impl From<&Request> for u8 {
    fn from(req: &Request) -> Self {
        match req {
            Request::AnnounceFileTransfer { .. } => 1,
            Request::UploadFile { .. } => 2,
        }
    }
}

impl Request {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        // Request Type
        buf.write_u8(self.into())?;

        match self {
            Request::AnnounceFileTransfer { filename, otp } => {
                // filename
                write_string(buf, filename)?;

                // password
                write_string(buf, otp)?;
            }
            Request::UploadFile {
                transfer_type,
                filename,
            } => {
                // transfer type
                buf.write_u8(transfer_type.into())?;

                // filename
                write_string(buf, filename)?;

                // file
                let mut file = File::open(filename).unwrap();
                let size = file.metadata().unwrap().len();
                let mut transfered: usize = 0;

                loop {
                    print_transfer_progress(transfered as u64, size);
                    if size == transfered as u64 {
                        break;
                    }

                    let mut buffer = [0; 1024];
                    transfered += file.read(&mut buffer).unwrap();
                    buf.write_all(&buffer).unwrap();
                }

                println!();
            }
        }
        Ok(())
    }

    pub fn deserialize(mut buf: &mut impl Read, address: &str) -> io::Result<Request> {
        match buf.read_u8()? {
            1 => {
                let filename = extract_string(&mut buf)?;
                let otp = extract_string(&mut buf)?;
                Ok(Request::AnnounceFileTransfer { filename, otp })
            }
            2 => {
                println!("INFO [{}]: Receiving file..", address);

                let transfer_type = buf.read_u8().unwrap().into();
                let mut filename = extract_string(&mut buf)?;

                match transfer_type {
                    TransferType::Normal | TransferType::Replace => {}
                    TransferType::KeepBoth => {
                        filename = format!("{}_{}", "2", filename);
                    }
                }

                let mut file = File::create(&filename).unwrap();
                io::copy(&mut buf, &mut file).unwrap();

                println!(
                    r#"INFO [{}]: Successfully transfered "{}"."#,
                    address, &filename
                );

                Ok(Request::UploadFile {
                    transfer_type,
                    filename,
                })
            }
            _ => todo!(),
        }
    }
}

fn print_transfer_progress(transfered: u64, file_size: u64) {
    let trans_kb = transfered / 1000;
    let file_size_kb = file_size / 1000;

    // for files bigger then 5000kb display in mb
    if file_size_kb > 5000 {
        print!(
            "\rUploading {:.2}/{:.2} MB...",
            trans_kb as f64 / 1000.0,
            file_size_kb as f64 / 1000.0
        );
    } else {
        print!("\rUploading {}/{} kB...", trans_kb, file_size_kb);
    }
}
