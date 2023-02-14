use glium::glutin;

pub fn handle_window_event (ev: glutin::event::WindowEvent, control_flow: &mut glutin::event_loop::ControlFlow) {
    match ev {
        glutin::event::WindowEvent::CloseRequested => control_flow.set_exit(),
        glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.state {
            glutin::event::ElementState::Pressed => match input.virtual_keycode {
                Some(glutin::event::VirtualKeyCode::Escape) => control_flow.set_exit(),
                _ => (),
            },
            _ => (),
        },
        _ => (),
    }
}
