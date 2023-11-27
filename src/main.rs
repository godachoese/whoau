use whoau::Setting;

mod server;

fn main() {
    let setting = Setting::default();
    server::run(&setting);
}
