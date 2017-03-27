extern crate pewpew;

use pewpew::engine::client::Client;

fn main() {
    let client = Client::connect();
    pewpew::engine::graphics::open_window();
}
