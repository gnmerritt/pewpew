extern crate sdl2;

use self::sdl2::event::Event;
use self::sdl2::keyboard::Keycode;

pub fn open_window() {
    let sdl_context = self::sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("SDL2", 800, 640)
        .position_centered().build().unwrap();

    let mut renderer = window.renderer()
        .accelerated().build().unwrap();

    renderer.set_draw_color(sdl2::pixels::Color::RGBA(1, 0, 0, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
           match event {
               Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                   running = false;
               },
               _ => {}
           }
       }

       renderer.clear();
       renderer.present();
    }
}
