use byteorder::{NetworkEndian, ReadBytesExt, WriteBytesExt};
use std::io::{self, Read, Write};
pub mod client;
pub mod server;

#[derive(Debug)]
pub enum Request {
    /// Echo back
    Echo(String),
    /// Jumbmle
    Jumble { message: String, amount: u16 },
}

#[derive(Debug)]
pub struct Response(pub String);

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
