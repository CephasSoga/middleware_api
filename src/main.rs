pub mod api;
pub mod server;
pub mod errors;

fn main() { // Updated return type
    let config: server::ServerConfig = server::ServerConfig::new(None);
    let r = config.parse();
    println!("{:?}", r);
}