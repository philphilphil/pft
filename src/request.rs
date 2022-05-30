use crate::extract_string;
use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug)]
pub enum Request {
    /// Echo back
    Echo(String),
    /// Jumbmle
    Jumble { message: String, amount: u16 },
}

impl From<&Request> for u8 {
    fn from(req: &Request) -> Self {
        match req {
            Request::Echo(_) => 1,
            Request::Jumble { .. } => 2,
        }
    }
}

impl Request {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        buf.write_u8(self.into())?;

        match self {
            Request::Echo(message) => {
                let message = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                buf.write_all(message)?;
            }
            Request::Jumble { message, amount } => {
                let message = message.as_bytes();
                buf.write_u16::<NetworkEndian>(message.len() as u16)?;
                buf.write_all(message)?;

                buf.write_u16::<NetworkEndian>(2)?;
                buf.write_u16::<NetworkEndian>(*amount)?;
            }
        }
        Ok(())
    }

    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Request> {
        match buf.read_u8()? {
            1 => Ok(Request::Echo(extract_string(&mut buf)?)),
            2 => {
                let message = extract_string(&mut buf)?;

                let _amount_len = buf.read_u16::<NetworkEndian>()?;
                let amount = buf.read_u16::<NetworkEndian>()?;
                Ok(Request::Jumble { message, amount })
            }
            _ => todo!(),
        }
    }
}
