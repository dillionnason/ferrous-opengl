use vecmath::{Vector3, vec3_normalized};

pub struct Camera {
    aspect_ratio: f32, 
    position: Vector3<f32>,
    pub up: Vector3<f32>,
    pub right: Vector3<f32>,
    pub forward: Vector3<f32>,
    dx: i32,
    dy: i32,
    velocity: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            aspect_ratio: 0f32, 
            position: [2.0, -1.0, 1.0],
            up:      [0f32, 1f32, 0f32],
            right:   [1f32, 0f32, 0f32],
            forward: [-2f32, 1f32, 1f32],
            dx:       0i32,
            dy:       0i32,
            velocity: 1.5f32,
        }
    }

    pub fn perspective_matrix(&mut self, width: f32, height: f32) -> [[f32; 4]; 4] {
        self.aspect_ratio = height as f32 / width as f32;

        let fov: f32 = 3.141592 / 3.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        [
            [ f * self.aspect_ratio  , 0.0,              0.0              ,  0.0 ],
            [       0.0         ,  f ,              0.0              ,  0.0 ],
            [       0.0         , 0.0,  (zfar+znear)/(zfar-znear)    ,  1.0 ],
            [       0.0         , 0.0, -(2.0*zfar*znear)/(zfar-znear),  0.0 ],
        ]
    }

    pub fn view_matrix(&self) -> [[f32; 4]; 4] {
        let f = vec3_normalized(self.forward);
        let up = self.up;
        let _0: f32 = 0f32;
        let _1: f32 = 1f32;

        let s = [f[1] * up[2] - f[2] * up[1],
             f[2] * up[0] - f[0] * up[2],
             f[0] * up[1] - f[1] * up[0]];

        let s_norm = vec3_normalized(s);

        let u = [s_norm[1] * f[2] - s_norm[2] * f[1],
             s_norm[2] * f[0] - s_norm[0] * f[2],
             s_norm[0] * f[1] - s_norm[1] * f[0]];

        let p = [-self.position[0] * s[0] - self.position[1] * s[1] - self.position[2] * s[2],
             -self.position[0] * u[0] - self.position[1] * u[1] - self.position[2] * u[2],
             -self.position[0] * f[0] - self.position[1] * f[1] - self.position[2] * f[2]];

        // note: remember that this is column-major, so the lines of code are actually columns
        [
          [s[0], u[0], -f[0], _0],
          [s[1], u[1], -f[1], _0],
          [s[2], u[2], -f[2], _0],
          [p[0], p[1],  p[2], _1],
        ]    
    }
}
