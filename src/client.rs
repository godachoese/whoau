use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use super::Message;

pub type ClientID = SocketAddr;

pub struct Client {
    pub no: usize,
    pub id: ClientID,
    stream: TcpStream,
}

impl Client {
    pub fn new(no: usize, stream: &TcpStream) -> Self {
        let id = stream.peer_addr().unwrap();
        let stream = stream.try_clone().unwrap();
        Self { no, id, stream }
    }
    pub fn send(&mut self, message: &Message) {
        self.stream.write_all(message.as_bytes()).unwrap_or(());
    }
    pub fn receive(&mut self) -> Option<Message> {
        let mut buffer = vec![0; 1024];
        let read = self.stream.read(&mut buffer).ok()?;
        if read == 0 {
            return None;
        }
        let trimmed = buffer[..read].to_vec();
        Message::from_utf8(trimmed).ok()
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self::new(self.no, &self.stream)
    }
}

impl Display for Client {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Client no.{}", self.no)
    }
}
