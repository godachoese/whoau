use std::collections::HashMap;
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use whoau::text::{self, Direction};
use whoau::timezone;
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

const BREAK: &str = "################################################";

fn join(client: Client, clients: &mut HashMap<ClientID, Client>) {
    let message = format!(
        "{BREAK}\n\
        {} joined\n\
        {} clients are in `Who Are You` now!\n\
        Please stay tuned\n\
        {BREAK}\n",
        client,
        clients.len() + 1
    );
    print!("{}-> {}", client.id, message,);
    clients.insert(client.id, client);
    for client in clients.values_mut() {
        client.send(&message);
    }
}

fn leave(id: ClientID, clients: &mut HashMap<ClientID, Client>) {
    if let Some(client) = clients.get(&id) {
        let message = format!(
            "{BREAK}\n\
            {} left\n\
            {BREAK}\n",
            client,
        );
        print!("{} | {}", client.id, message);
        for client in clients.values_mut() {
            client.send(&message);
        }
        clients.remove(&id);
    }
}

const PADDING: usize = 30;

fn relay(id: ClientID, message: Message, clients: &mut HashMap<ClientID, Client>) {
    if let Some(client) = clients.get(&id) {
        let message = text::align(message.trim(), Direction::Left, PADDING);
        let now = timezone::now_kst();
        let message = format!("{}    #no.{}|{}\n", message, client.no, now);
        for (client_id, client) in clients {
            if id != *client_id {
                client.send(&message);
            }
        }
        print!("{} -> {}", id, message);
    }
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
    for (no, stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();
        let sender = sender.clone();
        let client = Client::new(no, &stream);
        sender.send(ClientJoined(client.clone())).unwrap();
        thread::spawn(move || serve(client.clone(), sender));
    }
}
