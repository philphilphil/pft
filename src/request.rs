use crate::extract_string;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{self, Read, Write},
    net::SocketAddr,
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
                let mut file_buf = [0; 4096];

                let n = file.read(&mut file_buf).unwrap();
                buf.write_all(&file_buf[..n]).unwrap();
            }
        }
        Ok(())
    }

    pub fn deserialize(mut buf: &mut impl Read, address: &SocketAddr) -> io::Result<Request> {
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

                println!("INFO [{}]: Successfully transfered {}.", address, &filename);
                Ok(Request::UploadFile { filename })
            }
            _ => todo!(),
        }
    }
}
