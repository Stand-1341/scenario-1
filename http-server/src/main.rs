mod handlers;
mod http;
mod router;
mod server;

use router::Router;
use server::Server;

fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:3000".to_string();
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let public = std::env::var("PUBLIC_PATH")
        .unwrap_or_else(|_| format!("{crate_dir}/public"));
    let data = std::env::var("DATA_PATH").unwrap_or_else(|_| format!("{crate_dir}/data"));
    let router = Router::new(public, data);
    let mut server = Server::new(addr, router);
    server.run()
}
