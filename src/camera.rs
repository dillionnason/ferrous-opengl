extern crate glium;
use glium::glutin::event::{self, VirtualKeyCode, KeyboardInput, ElementState};
use glium::glutin::event::Event;
use vecmath::{Vector3, vec3_cross, vec3_normalized_sub};

pub struct FirstPersonSettings {
    pub move_forward: VirtualKeyCode,
    pub move_backward: VirtualKeyCode,
    pub strafe_left: VirtualKeyCode,
    pub strafe_right: VirtualKeyCode,
    pub fly_up: VirtualKeyCode,
    pub fly_down: VirtualKeyCode,
    pub move_faster: VirtualKeyCode,
    pub speed_horizontal: f32,
    pub speed_vertical: f32,
}

impl FirstPersonSettings {
    pub fn keyboard_wasd() -> Self {
        Self {
            move_forward: VirtualKeyCode::W,
            move_backward: VirtualKeyCode::S,
            strafe_left: VirtualKeyCode::A,
            strafe_right: VirtualKeyCode::D,
            fly_up: VirtualKeyCode::Space,
            fly_down: VirtualKeyCode::C,
            move_faster: VirtualKeyCode::LShift,
            speed_horizontal: 0f32,
            speed_vertical: 0f32,
        }
    }
}

pub struct Camera {
    aspect_ratio: f32,
    position: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub forward: Vector3<f32>,
    dx: f64, 
    dy: f64,
    yaw: f32,
    pitch: f32,
    settings: FirstPersonSettings,
    keys: Vec<VirtualKeyCode>,
}

impl Camera {
    pub fn new(height: f32, width: f32) -> Camera {
        Camera {
            aspect_ratio: height/width, 
            position: [2.0, -1.0, 1.0],
            right:   [1f32, 0f32, 0f32],
            up:      [0f32, 1f32, 0f32],
            forward: [-2f32, 1f32, 1f32],
            dx: 0f64,
            dy: 0f64,
            yaw: 0f32,
            pitch: 0f32,
            settings: FirstPersonSettings::keyboard_wasd(),
            keys: Vec::new(),
        }
    }

    pub fn push_keys(&mut self, key: &VirtualKeyCode) {
        if !self.keys.contains(key) {
            self.keys.push(*key)
        }
    }

    // Orients the camera to look at a point.
    pub fn look_at(&mut self, point: Vector3<f32>) {
        self.forward = vec3_normalized_sub(self.position, point);
        self.update_right();
    }

    pub fn perspective_matrix(&self) -> [[f32 ;4]; 4] {
        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [ f * self.aspect_ratio, 0.0,              0.0              ,  0.0 ],
            [       0.0            ,  f ,              0.0              ,  0.0 ],
            [       0.0            , 0.0,  (zfar+znear)/(zfar-znear)    ,  1.0 ],
            [       0.0            , 0.0, -(2.0*zfar*znear)/(zfar-znear),  0.0 ],
        ]
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.forward;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };

        let up = self.up;

        let s = [f[1] * up[2] - f[2] * up[1],
                 f[2] * up[0] - f[0] * up[2],
                 f[0] * up[1] - f[1] * up[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        }; 

        let u = [s_norm[1] * f[2] - s_norm[2] * f[1],
                 s_norm[2] * f[0] - s_norm[0] * f[2],
                 s_norm[0] * f[1] - s_norm[1] * f[0]];

        let p = [-self.position[0] * s[0] - self.position[1] * s_norm[1] - self.position[2] * s_norm[2],
                 -self.position[0] * u[0] - self.position[1] *    u[1]   - self.position[2] * u[2],
                 -self.position[0] * f[0] - self.position[1] *    f[1]   - self.position[2] * f[2]];
 
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [   p[0]  , p[1], p[2], 1.0],
        ] 
    }

    pub fn update(&mut self) {
        self.update_direction();

        // math debugging
        //print!("\x1B[2J\x1B[1;1H");
        //println!("Mouse movement: [{:?}, {:?}]", x, y);
        //println!("dx, dy:         [{:?}, {:?}]", self.dx, self.dy);
        //println!("forward:        {:?}", self.forward);
        //println!("right:          {:?}", self.right);
    }

    fn update_direction(&mut self) {
        let pi: f32 = std::f32::consts::PI;
        let _0 = 0f32;
        let _1 = 1f32;
        let _2 = _1 + _1;
        let _3 = _2 + _1;
        let _4 = _3 + _1;
        let _360 = 360f32;
        let dy: f32 = self.dy as f32;
        let dx: f32 = self.dx as f32;

        self.yaw = (self.yaw - dx / _360 * pi / _4) % (_2 * pi);
        self.pitch = self.pitch + dy / _360 * pi / _4;
        self.pitch = self.pitch.min(pi / _2).max(-pi / _2);
        
        //println!("{:?}, {:?}", self.yaw, self.pitch);

        self.set_yaw_pitch();
    }

    fn set_yaw_pitch(&mut self) {
        let (y_s, y_c, p_s, p_c) = (self.yaw.sin(), self.yaw.cos(), self.pitch.sin(), self.pitch.cos());
        self.forward = [y_s * p_c, p_s, y_c * p_c];
        self.up = [y_s * -p_s, p_c, y_c * -p_s];
        self.update_right();
    }    

    fn update_right(&mut self) {
        self.right = vec3_cross(self.up, self.forward);
    }

    pub fn parse_input(&mut self, e: &event::DeviceEvent) {
        match e {
            event::DeviceEvent::Key(KeyboardInput { state, virtual_keycode, .. }) => {
                let virtual_keycode = virtual_keycode.unwrap();
                match state {
                    ElementState::Pressed => {
                        if !self.keys.contains(&virtual_keycode) {
                            self.keys.push(virtual_keycode);
                        }
                    },
                    ElementState::Released => {
                        if self.keys.contains(&virtual_keycode) {
                            let i = self.keys.iter().position(|&r| r == virtual_keycode).unwrap();
                            self.keys.remove(i);
                        }
                    },
                    _ => {},
                }
            },
            event::DeviceEvent::MouseMotion { delta: (ref x, ref y) } => {
                self.dx = *x;
                self.dy = -y;
            },
            _ => {},
        }
    }
}
