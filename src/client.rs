use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

use super::Message;

pub type ClientID = SocketAddr;

pub struct Client {
    pub id: ClientID,
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: &TcpStream) -> Self {
        let id = stream.peer_addr().unwrap();
        let stream = stream.try_clone().unwrap();
        Self { id, stream }
    }
    pub fn send(&mut self, message: &Message) {
        self.stream.write_all(message.as_bytes()).unwrap_or(());
    }
    pub fn receive(&mut self) -> Option<Message> {
        let mut buffer = vec![0; 1024];
        let read = self.stream.read(&mut buffer);
        match read {
            Ok(n) => match n {
                0 => None,
                _ => match Message::from_utf8(buffer) {
                    Ok(message) => Some(message),
                    Err(_) => None,
                },
            },
            Err(_) => None,
        }
    }
}

impl Clone for Client {
    fn clone(&self) -> Self {
        Self::new(&self.stream)
    }
}
