mod commands;
mod database;
mod server;
mod logger;

use server::Server;

fn main() {
    let server = Server::new("0.0.0.0:8888");
    server.run();
}
