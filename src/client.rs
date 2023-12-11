use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    pub name: String,
    stream: TcpStream,
}

impl Client {
    pub fn new(no: usize, stream: &TcpStream) -> Self {
        Self {
            name: format!("anonymous{}", no),
            stream: stream.try_clone().expect("Connect closed"),
        }
    }
    pub fn send(&mut self, message: &String) {
        self.stream.write_all(message.as_bytes()).unwrap_or(());
    }
    pub fn receive(&mut self) -> Option<String> {
        let mut buffer = vec![0; 1024];
        let read = self.stream.read(&mut buffer).ok()?;
        if read == 0 {
            return None;
        }
        let trimmed = buffer[..read].to_vec();
        String::from_utf8(trimmed).ok()
    }

    pub fn try_clone(&self) -> Result<Self, String> {
        match self.stream.try_clone() {
            Ok(stream) => Ok(Self {
                name: self.name.clone(),
                stream,
            }),
            Err(_) => Err("Connection closed".to_string()),
        }
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "client:{}", self.name)
    }
}

impl PartialEq for Client {
    fn eq(&self, other: &Self) -> bool {
        self.stream.peer_addr().expect("No connection")
            == other.stream.peer_addr().expect("No connection")
    }
}

impl Eq for Client {}
