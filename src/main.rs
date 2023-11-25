use std::env;
use std::net::Ipv4Addr;

mod server;

const LOCAL_HOST: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

fn main() {
    let port = env::var("WHOAU_PORT").expect("Env 'WHOAU_PORT' is missed");
    server::run(LOCAL_HOST, port);
}
