#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

use gfx::traits::FactoryExt;
use gfx::Device;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "coord2d",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.8, -0.8 ] },
    Vertex { pos: [  0.8, -0.8 ] },
    Vertex { pos: [  0.0,  0.8 ] }
];

fn main() {
    let builder = glutin::WindowBuilder::new()
        .with_title("My First Triangle".to_string())
        .with_dimensions(640, 480);

    let events_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, _main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);

    let mut encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();

    let pso = factory.create_pipeline_simple(
        include_bytes!("triangle_120.glslv"),
        include_bytes!("triangle_120.glslf"),
        pipe::new()
    ).unwrap();

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());

    let data = pipe::Data {
        vbuf: vertex_buffer,
        out: main_color,
    };

    let mut running = true;
    while running {
        events_loop.poll_events(|glutin::Event::WindowEvent{window_id: _, event}| {
            match event {
                glutin::WindowEvent::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape), _) |
                glutin::WindowEvent::Closed => running = false,
                _ => ()
            }
        });

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
