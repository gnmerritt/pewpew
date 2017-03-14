#[macro_use]
extern crate gfx;
extern crate gfx_window_sdl;
extern crate nalgebra;
extern crate nphysics3d;
extern crate sdl2;

pub mod engine;

fn main() {
    engine::graphics::open_window();
}
