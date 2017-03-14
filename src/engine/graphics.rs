use gfx;
use gfx_window_sdl;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use gfx::Device;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [Vertex {
                                   pos: [-0.5, -0.5],
                                   color: [1.0, 0.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.5, -0.5],
                                   color: [0.0, 1.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.0, 0.5],
                                   color: [0.0, 0.0, 1.0],
                               }];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub fn open_window() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let builder = video_subsystem.window("SDL2", 800, 640);

    let (window, _, mut device, mut factory, color_view, _) =
        gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder).unwrap();
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("shader/triangle_150.glslv"),
        include_bytes!("shader/triangle_150.glslf"),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: color_view,
    };

    'main: loop {
        let mut event_pump = sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
           match event {
               Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                   break 'main;
               },
               _ => {}
           }
       }

       // draw a frame
       encoder.clear(&data.out, CLEAR_COLOR);
       encoder.draw(&slice, &pso, &data);
       encoder.flush(&mut device);
       window.gl_swap_window();
       device.cleanup();
    }
}
