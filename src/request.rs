use crate::extract_string;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::{
    fs::File,
    io::{self, Read, Write},
};

#[derive(Debug)]
pub enum Request {
    /// Test if OTP password is valid
    TestOTP(String),
    /// Jumbmle
    UploadFile { filename: String },
}

impl From<&Request> for u8 {
    fn from(req: &Request) -> Self {
        match req {
            Request::TestOTP(_) => 1,
            Request::UploadFile { .. } => 2,
        }
    }
}

impl Request {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u8(self.into())?;

        match self {
            Request::TestOTP(message) => {
                let message = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                buf.write_all(message)?;
            }
            Request::UploadFile { filename } => {
                let message = filename.as_bytes();
                buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                buf.write_all(message)?;

                let mut file = File::open(filename).unwrap();
                let mut file_buf = [0; 4096];

                let n = file.read(&mut file_buf).unwrap();
                buf.write_all(&file_buf[..n]).unwrap();
            }
        }
        Ok(())
    }

    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Request> {
        match buf.read_u8()? {
            1 => Ok(Request::TestOTP(extract_string(&mut buf)?)),
            2 => {
                println!("Receiving file..");
                let filename = format!("server/{}", extract_string(&mut buf)?);
                println!("a");
                let mut file = File::create(&filename).unwrap();
                println!("c");

                loop {
                    io::copy(&mut buf, &mut file).unwrap();

                    if buf.bytes().count() == 0 {
                        break;
                    }
                }
                println!("d");

                println!("Successfully transfered file {}.", &filename);
                Ok(Request::UploadFile { filename })
            }
            _ => todo!(),
        }
    }
}
