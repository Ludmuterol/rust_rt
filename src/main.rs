use std::time::Instant;

use glium::{Surface, glutin::dpi::PhysicalSize};

extern crate glium;

mod event_handler;
mod renderer;
mod parser;
mod vec3;
mod objects;
mod solve_quadratic;
mod camera;
mod aabb;

fn main() {
    use glium::glutin;

    let scene = parser::parse();
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(PhysicalSize{width: scene.width, height: scene.height});
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap(); 
    let target = display.draw();
    let now = Instant::now();
    let pixels = renderer::render(&scene);
    let elapsed = now.elapsed();
    println!("Running render() took {} seconds.", elapsed.as_secs_f32());
    let converted_pixels = glium::texture::RawImage2d::from_raw_rgb(pixels, (scene.width, scene.height));
    glium::Texture2d::new(&display, converted_pixels)
        .unwrap()
        .as_surface()
        .fill(&target, glium::uniforms::MagnifySamplerFilter::Nearest);
    target.finish().unwrap();
    event_loop.run(move |ev, _, control_flow| {
        
        let next_frame_time = std::time::Instant::now() + 
            std::time::Duration::from_nanos(1_000_000_000 / 60);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => event_handler::handle_window_event(event, control_flow),
            glutin::event::Event::MainEventsCleared => (),
            _ => (),
        }
    });
}
