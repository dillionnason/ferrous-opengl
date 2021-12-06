use glium::glutin::event::VirtualKeyCode;

// First person camera settings.
pub struct FirstPersonSettings {
    // Which button to press to move forward.
    pub move_forward_button: VirtualKeyCode,
    // Which button to press to move backward.
    pub move_backward_button: VirtualKeyCode,
    // Which button to press to strafe left.
    pub strafe_left_button: VirtualKeyCode,
    // Which button to press to strafe right.
    pub strafe_right_button: VirtualKeyCode,
    // Which button to press to fly up.
    pub fly_up_button: VirtualKeyCode,
    // Which button to press to fly down.
    pub fly_down_button: VirtualKeyCode,
    // Which button to press to move faster.
    pub move_faster_button: VirtualKeyCode,
    // The horizontal movement speed. units per second.
    pub speed_horizontal: f32,
    // The vertical movement speed. units per second.
    pub speed_vertical: f32,
}

impl FirstPersonSettings {
    // Creates new first person camera settings with wasd defaults.
    pub fn keyboard_wasd() -> FirstPersonSettings {
        FirstPersonSettings {
            move_forward_button: VirtualKeyCode::Space,
            move_backward_button: VirtualKeyCode::LShift,
            strafe_left_button: VirtualKeyCode::A,
            strafe_right_button: VirtualKeyCode::D,
            fly_up_button: VirtualKeyCode::W,
            fly_down_button: VirtualKeyCode::S,
            move_faster_button: VirtualKeyCode::Return,
            speed_horizontal: 0f32,
            speed_vertical: 0f32,
        }
    }
    pub fn keyboard_arrows() -> FirstPersonSettings {
        FirstPersonSettings {
            move_forward_button: VirtualKeyCode::Space,
            move_backward_button: VirtualKeyCode::LShift,
            strafe_left_button: VirtualKeyCode::Left,
            strafe_right_button: VirtualKeyCode::Right,
            fly_up_button: VirtualKeyCode::Up,
            fly_down_button: VirtualKeyCode::Down,
            move_faster_button: VirtualKeyCode::Return,
            speed_horizontal: 0f32,
            speed_vertical: 0f32,
        }
    }
}

pub struct CameraState {
    aspect_ratio: f32,
    position: Vector3<f32>,
    yaw: f32,
    pitch: f32,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub forward: Vector3<f32>,
    keys: Vec<VirtualKeyCode>,
    mouse: Option<(i32, i32)>,
    dx: i32,
    dy: i32,
    velocity: f32,
    fast_scale: f32,
    settings: FirstPersonSettings,
}

impl CameraState {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1024.0 / 768.0,
            position: [0.0, 0.0, 6.0],
            right:   [1f32, 0f32, 0f32],
            up:      [0f32, 1f32, 0f32],
            forward: [0f32, 0f32, 1f32],
            keys: Vec::new(),
            mouse: None,
            dx: 0i32,
            dy: 0i32,
            yaw: 0f32,
            pitch: 0f32,
            velocity: 1.5f32,
            fast_scale: 5f32,
            settings: FirstPersonSettings::keyboard_wasd(),
        }
    }
}
