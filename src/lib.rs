pub mod client;
pub mod server;
pub mod setting;
pub mod text;
pub mod timezone;

use setting::Setting;
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

pub fn run(setting: &Setting) {
    let listener = TcpListener::bind(setting.addr).unwrap();
    println!("Server is listening on {}", listener.local_addr().unwrap());
    let (sender, receiver) = mpsc::channel::<server::Packet>();
    thread::spawn(|| server::broadcast(receiver));
    for (no, stream) in listener.incoming().enumerate() {
        let stream = stream.unwrap();
        let client = client::Client::new(no, &stream);
        if let Ok(client) = client.try_clone() {
            let sender = sender.clone();
            thread::spawn(move || server::serve(client, sender));
        }
    }
}
