extern crate pewpew;
use pewpew::engine::networking::Server;

fn main() {
    Server::listen(); // this blocks until the server is shut down
}
