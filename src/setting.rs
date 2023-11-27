use std::env;
use std::net::{Ipv4Addr, SocketAddr};

pub struct Setting {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub addr: SocketAddr,
}

impl Default for Setting {
    fn default() -> Self {
        let ip = Ipv4Addr::new(0, 0, 0, 0);
        let port = env::var("WHOAU_PORT")
            .expect("WHOAU_PORT is missed")
            .parse()
            .expect("Wrong format");
        let addr = SocketAddr::from((ip, port));
        Self { ip, port, addr }
    }
}
