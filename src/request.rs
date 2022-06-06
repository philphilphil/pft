use crate::extract_string;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{self, Read, Write},
};

#[derive(Debug)]
pub enum Request {
    /// Announce file transfer to server
    AnnounceFileTransfer { filename: String, otp: String },
    /// Upload File
    UploadFile { filename: String },
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
        buf.write_u8(self.into())?;

        match self {
            Request::AnnounceFileTransfer { filename, otp } => {
                let filename = filename.as_bytes();
                buf.write_u16::<NetworkEndian>(filename.len() as u16)?;
                buf.write_all(filename)?;

                let otp = otp.as_bytes();
                buf.write_u16::<NetworkEndian>(otp.len() as u16)?;
                buf.write_all(otp)?;
            }
            Request::UploadFile { filename } => {
                let filename_bytes = filename.as_bytes();
                buf.write_u16::<NetworkEndian>(filename_bytes.len() as u16)?;
                buf.write_all(filename_bytes)?;

                let mut file = File::open(filename).unwrap();
                let size = file.metadata().unwrap().len();
                let mut transfered: usize = 0;
                // let mut file_buf = vec![0; size as usize];

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
                let filename = format!("server/{}", extract_string(&mut buf)?);
                let otp = extract_string(&mut buf)?;
                Ok(Request::AnnounceFileTransfer { filename, otp })
            }
            2 => {
                println!("INFO [{}]: Receiving file..", address);
                let filename = format!("server/{}", extract_string(&mut buf)?);
                let mut file = File::create(&filename).unwrap();

                io::copy(&mut buf, &mut file).unwrap();

                println!(
                    r#"INFO [{}]: Successfully transfered "{}"."#,
                    address, &filename
                );
                Ok(Request::UploadFile { filename })
            }
            _ => todo!(),
        }
    }
}

fn print_transfer_progress(transfered: u64, file_size: u64) {
    let trans_kb = transfered / 1000;
    let file_size_kb = file_size / 1000;

    if file_size_kb > 1000 {
        print!(
            "\rUploading {:.2}/{:.2} MB...",
            trans_kb as f64 / 1000.0,
            file_size_kb as f64 / 1000.0
        );
    } else {
        print!("\rUploading {}/{} kB...", trans_kb, file_size_kb);
    }
}
