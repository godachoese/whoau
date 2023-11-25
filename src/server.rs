use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::net::{SocketAddr, TcpListener};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use whoau::{Client, ClientID, Message};

enum Packet {
    ClientJoined(Client),
    ClientLeft { id: ClientID },
    MessageReceived { id: ClientID, message: Message },
}

fn serve(mut client: Client, sender: Sender<Packet>) {
    loop {
        let received = client.receive();
        match received {
            Some(message) => {
                print!("{}", message);
                let packet = Packet::MessageReceived {
                    id: client.id,
                    message,
                };
                sender.send(packet).unwrap();
            }
            None => {
                let packet = Packet::ClientLeft { id: client.id };
                sender.send(packet).unwrap();
                break;
            }
        }
    }
}

fn broadcast(receiver: Receiver<Packet>) {
    let mut clients = HashMap::<ClientID, Client>::new();
    for packet in receiver {
        match packet {
            Packet::ClientJoined(client) => {
                println!("Client joined {}", client.id);
                clients.insert(client.id, client);
            }
            Packet::ClientLeft { id } => {
                println!("Client left {}", id);
                clients.remove(&id);
            }
            Packet::MessageReceived { id, message } => {
                for (client_id, client) in &mut clients {
                    if id != *client_id {
                        client.send(&message);
                    }
                }
            }
        }
    }
}

pub fn run(ip: Ipv4Addr, port: String) {
    let port = port.parse().expect("Wrong format");
    let addr = SocketAddr::from((ip, port));
    let listener = TcpListener::bind(addr).unwrap();
    println!("Server is listening on {}", listener.local_addr().unwrap());
    let (sender, receiver) = mpsc::channel::<Packet>();
    thread::spawn(|| broadcast(receiver));
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sender = sender.clone();
        let client = Client::new(&stream);
        sender.send(Packet::ClientJoined(client.clone())).unwrap();
        thread::spawn(move || serve(client.clone(), sender));
    }
}
