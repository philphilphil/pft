use std::{
    io::{self, BufRead, Write},
    net::TcpStream,
};

pub mod client;
pub mod server;

pub struct LinesCodec {
    reader: io::BufReader<TcpStream>,
    writer: io::LineWriter<TcpStream>,
}

impl LinesCodec {
    pub fn new(stream: TcpStream) -> io::Result<Self> {
        let writer = io::LineWriter::new(stream.try_clone()?);
        let reader = io::BufReader::new(stream);
        Ok(Self { reader, writer })
    }

    pub fn send_message(&mut self, message: &str) -> io::Result<()> {
        self.writer.write_all(message.as_bytes())?;
        self.writer.write_all(&[b'\n'])?;
        Ok(())
    }

    pub fn read_message(&mut self) -> io::Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop(); //remove trailing \n

        Ok(line)
    }
}
