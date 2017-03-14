#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate nalgebra;
extern crate nphysics3d;

pub mod engine;

fn main() {
    engine::graphics::open_window();
}
