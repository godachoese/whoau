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

const BREAK_BEGIN: &str = "################################################";
const BREAK_END: &str = "================================================";

fn join(mut client: Client, clients: &mut HashMap<ClientID, Client>, logs: &mut Vec<Message>) {
    let message = format!(
        "{BREAK_BEGIN}\n\
        {} joined\n\
        {} clients are in `Who Are You` now!\n\
        Please stay tuned\n\
        {BREAK_END}\n",
        client,
        clients.len() + 1
    );
    print!("{}-> {}", client.id, message,);
    for client in clients.values_mut() {
        client.send(&message);
    }
    logs.push(message);
    for log in logs {
        client.send(log)
    }
    clients.insert(client.id, client);
}

fn leave(id: ClientID, clients: &mut HashMap<ClientID, Client>, logs: &mut Vec<Message>) {
    if let Some(client) = clients.get(&id) {
        let message = format!(
            "{BREAK_BEGIN}\n\
            {} left\n\
            {BREAK_END}\n",
            client,
        );
        print!("{} | {}", client.id, message);
        for client in clients.values_mut() {
            client.send(&message);
        }
        logs.push(message);
        clients.remove(&id);
    }
}

const PADDING: usize = 50;

fn relay(
    id: ClientID,
    message: Message,
    clients: &mut HashMap<ClientID, Client>,
    logs: &mut Vec<Message>,
) {
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
        logs.push(message);
    }
}

fn broadcast(receiver: Receiver<Packet>) {
    let mut logs = Vec::<Message>::new();
    let mut clients = HashMap::<ClientID, Client>::new();
    for packet in receiver {
        match packet {
            ClientJoined(client) => join(client, &mut clients, &mut logs),
            ClientLeft { id } => leave(id, &mut clients, &mut logs),
            MessageReceived { id, message } => relay(id, message, &mut clients, &mut logs),
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
