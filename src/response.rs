use crate::extract_string;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct Response {
    pub message: String,
    pub error: Option<FileTransferError>,
}

#[derive(Debug)]
pub enum FileTransferError {
    InvalidOneTimePassword,
    FileAlreadyExists,
}

impl From<&FileTransferError> for u8 {
    fn from(error: &FileTransferError) -> Self {
        match error {
            FileTransferError::InvalidOneTimePassword => 1,
            FileTransferError::FileAlreadyExists => 2,
        }
    }
}

impl From<u8> for FileTransferError {
    fn from(e: u8) -> Self {
        match e {
            1 => FileTransferError::InvalidOneTimePassword,
            2 => FileTransferError::FileAlreadyExists,
            _ => panic!("Error deserializin."),
        }
    }
}

impl Response {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        let message = self.message.as_bytes();
        buf.write_u16::<NetworkEndian>(message.len() as u16)?;
        buf.write_all(message)?;

        if let Some(e) = &self.error {
            buf.write_u8(1)?;
            buf.write_u8(e.into())?;
        } else {
            buf.write_u8(0)?;
        }

        Ok(())
    }

    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Response> {
        let message = extract_string(&mut buf)?;

        let contains_error = buf.read_u8()?;
        let error = if contains_error == 1 {
            Some(buf.read_u8().unwrap().into())
        } else {
            None
        };

        Ok(Response { message, error })
    }
}
