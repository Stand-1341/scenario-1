mod handlers;
mod http;
mod router;
mod server;

use router::Router;
use server::Server;

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:3000".to_string();
    let public = std::env::var("PUBLIC_PATH").unwrap_or_else(|_| "public".to_string());
    let data = std::env::var("DATA_PATH").unwrap_or_else(|_| "data".to_string());
    let router = Router::new(public, data);
    let mut server = Server::new(addr, router);
    server.run()
}
