use glium::{Surface, glutin::dpi::PhysicalSize};

extern crate glium;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 512;

mod event_handler;
mod renderer;
mod parser;
mod vec3;
mod objects;
mod solve_quadratic;

fn main() {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(PhysicalSize{width: WIDTH, height: HEIGHT});
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap(); 
    parser::parse();
    event_loop.run(move |ev, _, control_flow| {
        
        let next_frame_time = std::time::Instant::now() + 
            std::time::Duration::from_nanos(1_000_000_000 / 60);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => event_handler::handle_window_event(event, control_flow),
            glutin::event::Event::MainEventsCleared => {
                let target = display.draw();
                let pixels = renderer::render(WIDTH, HEIGHT);
                let converted_pixels = glium::texture::RawImage2d::from_raw_rgb(pixels, (WIDTH, HEIGHT));
                glium::Texture2d::new(&display, converted_pixels)
                    .unwrap()
                    .as_surface()
                    .fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);

                    target.finish().unwrap();

            },
            _ => (),
        }
    });
}
