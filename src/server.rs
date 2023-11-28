use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use whoau::{Client, ClientID, Message, Setting};
use Packet::{ClientJoined, ClientLeft, MessageReceived};

enum Packet {
    ClientJoined(Client),
    ClientLeft { id: ClientID },
    MessageReceived { id: ClientID, message: Message },
}

fn serve(mut client: Client, sender: Sender<Packet>) {
    while let Some(message) = client.receive() {
        let packet = MessageReceived {
            id: client.id,
            message,
        };
        sender.send(packet).unwrap();
    }
    let packet = ClientLeft { id: client.id };
    sender.send(packet).unwrap();
}

fn join(client: Client, clients: &mut HashMap<ClientID, Client>) {
    let id = client.id;
    let message = "Client joined\n".to_string();
    clients.insert(client.id, client);
    for client in clients.values_mut() {
        client.send(&message);
    }
    print!("{} {}", message, id);
}

fn leave(id: ClientID, clients: &mut HashMap<ClientID, Client>) {
    let message = format!("Client left {}\n", id);
    for client in clients.values_mut() {
        client.send(&message);
    }
    clients.remove(&id);
    print!("{}", message);
}

fn relay(id: ClientID, message: Message, clients: &mut HashMap<ClientID, Client>) {
    for (client_id, client) in clients {
        if id != *client_id {
            client.send(&message);
        }
    }
    print!("{}, {}", id, message);
}

fn broadcast(receiver: Receiver<Packet>) {
    let mut clients = HashMap::<ClientID, Client>::new();
    for packet in receiver {
        match packet {
            ClientJoined(client) => join(client, &mut clients),
            ClientLeft { id } => leave(id, &mut clients),
            MessageReceived { id, message } => relay(id, message, &mut clients),
        };
    }
}

pub fn run(setting: &Setting) {
    let listener = TcpListener::bind(setting.addr).unwrap();
    println!("Server is listening on {}", listener.local_addr().unwrap());
    let (sender, receiver) = mpsc::channel::<Packet>();
    thread::spawn(|| broadcast(receiver));
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let sender = sender.clone();
        let client = Client::new(&stream);
        sender.send(ClientJoined(client.clone())).unwrap();
        thread::spawn(move || serve(client.clone(), sender));
    }
}
