use std::sync::mpsc::{Receiver, Sender};

use message::Message;

use super::client::Client;
use super::text::{self, Direction};
use super::timezone;

mod message;

pub enum Packet {
    Joined { client: Client },
    Left { client: Client },
    Text { client: Client, text: String },
}

pub fn serve(mut client: Client, sender: Sender<Packet>) -> Result<(), String> {
    sender
        .send(Packet::Joined {
            client: client.try_clone()?,
        })
        .unwrap();

    while let Some(data) = client.receive() {
        match Message::parse(data) {
            Message::Text(text) => {
                let packet = Packet::Text {
                    client: client.try_clone()?,
                    text,
                };
                sender.send(packet).unwrap();
            }
            Message::ChangeName(name) => {
                client.name = name;
            }
            Message::Error(_) => (),
        }
    }
    let packet = Packet::Left {
        client: client.try_clone()?,
    };
    sender.send(packet).unwrap();
    Ok(())
}

const BREAK_BEGIN: &str = "################################################";
const BREAK_END: &str = "================================================";

pub fn join(mut client: Client, clients: &mut Vec<Client>, logs: &mut Vec<String>) {
    let text = format!(
        "{BREAK_BEGIN}\n\
        {} joined\n\
        {} clients are in `Who Are You` now!\n\
        Please stay tuned\n\
        {BREAK_END}\n",
        client,
        clients.len() + 1
    );
    print!("{}-> {}", client, text);
    for client in clients.iter_mut() {
        client.send(&text);
    }
    for log in logs.iter_mut() {
        client.send(log)
    }
    client.send(&text);
    clients.push(client);
    logs.push(text);
}

fn leave(client: Client, clients: &mut Vec<Client>, logs: &mut Vec<String>) {
    let message = format!(
        "{BREAK_BEGIN}\n\
            {} left\n\
            {BREAK_END}\n",
        client,
    );
    for i in 0..clients.len() {
        if clients[i] == client {
            clients.remove(i);
        } else {
            clients[i].send(&message);
        }
    }
    logs.push(message);
}

const PADDING: usize = 50;

pub fn relay(client: &Client, mut text: String, clients: &mut Vec<Client>, logs: &mut Vec<String>) {
    text = text::align(text.trim(), Direction::Left, PADDING);
    text = format!("{}    #{}|{}\n", text, client.name, timezone::now_kst());
    for other in clients {
        if other == client {
            continue;
        }
        other.send(&text);
    }
    print!("{} -> {}", client, text);
    logs.push(text);
}

pub fn broadcast(receiver: Receiver<Packet>) {
    let mut logs = Vec::<String>::new();
    let mut clients = Vec::new();
    for packet in receiver {
        match packet {
            Packet::Joined { client } => join(client, &mut clients, &mut logs),
            Packet::Left { client } => leave(client, &mut clients, &mut logs),
            Packet::Text { client, text } => relay(&client, text, &mut clients, &mut logs),
        };
    }
}
