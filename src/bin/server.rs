extern crate pewpew;
use pewpew::engine::networking;
use pewpew::engine::engine;

fn main() {
    let round = engine::Round::new();
    networking::launch_server(round); // this blocks until the server is shut down
}
