use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};

pub mod client;
pub mod request;
pub mod response;
pub mod server;

fn extract_string(buf: &mut impl Read) -> io::Result<String> {
    let length = buf.read_u16::<NetworkEndian>()?;

    let mut bytes = vec![0u8; length as usize];
    buf.read_exact(&mut bytes)?;

    String::from_utf8(bytes).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid utf8"))
}

fn write_string(buf: &mut impl Write, string: &String) -> io::Result<()> {
    let string_bytes = string.as_bytes();
    buf.write_u16::<NetworkEndian>(string_bytes.len() as u16)?;
    buf.write_all(string_bytes)?;
    Ok(())
}
