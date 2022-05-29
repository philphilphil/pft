use crate::request::Request;
use byteorder::{NetworkEndian, ReadBytesExt};
use std::io::{self, Read};
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

#[test]
fn test_request_roundtrip() {
    let req = Request::Echo(String::from("Hello"));

    let mut bytes: Vec<u8> = vec![];
    req.serialize(&mut bytes).unwrap();

    let mut reader = io::Cursor::new(bytes); // Simulating TcpStream
    let roundtrip_req = Request::deserialize(&mut reader).unwrap();

    assert!(matches!(roundtrip_req, Request::Echo(_)));
}
