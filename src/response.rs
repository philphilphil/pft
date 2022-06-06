use crate::{extract_string, write_string};
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

pub struct Response {
    pub message: String,
    pub error: Option<FileTransferError>,
}

pub enum FileTransferError {
    InvalidPassword,
    FileAlreadyExists,
}

impl From<&FileTransferError> for u8 {
    fn from(error: &FileTransferError) -> Self {
        match error {
            FileTransferError::InvalidPassword => 1,
            FileTransferError::FileAlreadyExists => 2,
        }
    }
}

impl From<u8> for FileTransferError {
    fn from(e: u8) -> Self {
        match e {
            1 => FileTransferError::InvalidPassword,
            2 => FileTransferError::FileAlreadyExists,
            _ => panic!("Error deserializing."),
        }
    }
}

impl Response {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        write_string(buf, &self.message)?;

        if let Some(e) = &self.error {
            buf.write_u8(e.into())?;
        } else {
            buf.write_u8(0)?;
        }

        Ok(())
    }

    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Response> {
        let message = extract_string(&mut buf)?;

        let pos_error = buf.read_u8()?;

        let error = if pos_error == 0 {
            None
        } else {
            Some(pos_error.into())
        };

        Ok(Response { message, error })
    }
}
