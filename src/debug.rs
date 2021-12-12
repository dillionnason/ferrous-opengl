extern crate glium;

pub struct Debug {
    keys: Vec<glium::glutin::event::VirtualKeyCode>,
}

impl Debug {
    pub fn new() -> Self {
        Self {
            keys: Vec::new(),
        }
    }

    pub fn get_draw_state(&self) -> glium::draw_parameters::PolygonMode {
        if self.keys.contains(&glium::glutin::event::VirtualKeyCode::T) {
            glium::draw_parameters::PolygonMode::Line
        } 
        else {
            glium::draw_parameters::PolygonMode::Fill
        }
    }

    pub fn parse_input(&mut self, e: &glium::glutin::event::DeviceEvent) {
        match e {
            glium::glutin::event::DeviceEvent::Key(glium::glutin::event::KeyboardInput { state, virtual_keycode, .. }) => {
                let virtual_keycode = virtual_keycode.unwrap();
                match state {
                    glium::glutin::event::ElementState::Pressed => {
                        if !self.keys.contains(&virtual_keycode) {
                            self.keys.push(virtual_keycode);
                        }
                    }
                    glium::glutin::event::ElementState::Released => {
                        if self.keys.contains(&virtual_keycode) {
                            let i = self.keys.iter().position(|&r| r == virtual_keycode).unwrap();
                            self.keys.remove(i);
                        }
                    }
                }
            },
            _ => (),
        }
    }
}