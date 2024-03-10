mod handler;
mod router;
mod server;

use server::Server;

fn main() {
    // start a server
    let server = Server::new("localhost:3000");
    // run the server
    server.run();
}
