use std::time::Duration;

use glium::{Surface, glutin::dpi::PhysicalSize};

extern crate glium;

const SCREEN_SIZE: PhysicalSize<u32> = PhysicalSize{ height: 512 , width: 512 };

fn main() {
    use glium::glutin;

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_inner_size(SCREEN_SIZE);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap(); 
    
    event_loop.run(move |ev, _, control_flow| {
        
        let next_frame_time = std::time::Instant::now() + 
            std::time::Duration::from_nanos(1_000_000_000 / 60);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => control_flow.set_exit(),
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
                    glutin::event::ElementState::Pressed => match input.virtual_keycode {
                        Some(glutin::event::VirtualKeyCode::Escape) => control_flow.set_exit(),
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },
            glutin::event::Event::MainEventsCleared => {
                let target = display.draw();
                let mut pixels = vec![0u8; (SCREEN_SIZE.width * SCREEN_SIZE.height * 3).try_into().unwrap()];
                let converted_pixels = glium::texture::RawImage2d::from_raw_rgb(pixels, SCREEN_SIZE.into());
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
