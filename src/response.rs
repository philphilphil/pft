use crate::extract_string;
use byteorder::{NetworkEndian, WriteBytesExt};
use std::io::{self, Read, Write};

#[derive(Debug)]
pub struct Response(pub String);

impl Response {
    pub fn serialize(&self, buf: &mut impl Write) -> io::Result<()> {
        let message = self.0.as_bytes();
        buf.write_u16::<NetworkEndian>(message.len() as u16)?;
        buf.write_all(message)?;
        Ok(())
    }
    pub fn deserialize(mut buf: &mut impl Read) -> io::Result<Response> {
        Ok(Response(extract_string(&mut buf).unwrap()))
    }
}
