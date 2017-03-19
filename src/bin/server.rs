extern crate pewpew;
use pewpew::engine::networking::Server;

fn main() {
    let mut server = Server::listen();
    server.accept_connections();
}
