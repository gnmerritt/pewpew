extern crate pewpew;
use pewpew::engine::networking;

fn main() {
    networking::launch_server(); // this blocks until the server is shut down
}
