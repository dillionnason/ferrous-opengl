use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    color: (f32, f32, f32, f32),
}

implement_vertex!(Vertex, position, color);

const _NORTH: (f32, f32, f32, f32) = (0.0, 1.0, 0.0, 1.0);
const _SOUTH: (f32, f32, f32, f32) = (0.0, -1.0, 0.0, 1.0);
const _EAST: (f32, f32, f32, f32) = (1.0, 0.0, 0.0, 1.0);
const _WEST: (f32, f32, f32, f32) = (-1.0, 0.0, 0.0, 1.0);
const _UP: (f32, f32, f32, f32) = (0.0, 0.0, 1.0, 1.0);
const _DOWN: (f32, f32, f32, f32) = (0.0, 0.0, -1.0, 1.0);


pub const VERTICES: [Vertex; 25] = [
    Vertex { position: (0.0, 0.0, 0.0), color: (0.0, 0.0, 0.0, 0.0) }, //dummy vertex because i'm dumb

    Vertex { position: (-0.5, -0.5, 0.0), color: (1.0, 0.0, 0.0, 1.0) }, //south (front)   // 1 
    Vertex { position: (-0.5,  0.5, 0.0), color: (1.0, 0.0, 0.0, 1.0) }, //south (front)   // 2
    Vertex { position: ( 0.5,  0.5, 0.0), color: (1.0, 0.0, 0.0, 1.0) }, //south (front)   // 3
    Vertex { position: ( 0.5, -0.5, 0.0), color: (1.0, 0.0, 0.0, 1.0) }, //south (front)   // 4

    Vertex { position: (-0.5, -0.5, 1.0), color: (1.0, 0.0, 0.0, 1.0) }, //north (back)    // 5
    Vertex { position: (-0.5,  0.5, 1.0), color: (1.0, 0.0, 0.0, 1.0) }, //north (back)    // 6
    Vertex { position: ( 0.5,  0.5, 1.0), color: (1.0, 0.0, 0.0, 1.0) }, //north (back)    // 7
    Vertex { position: ( 0.5, -0.5, 1.0), color: (1.0, 0.0, 0.0, 1.0) }, //north (back)    // 8

    Vertex { position: ( 0.5,  0.5, 0.0), color: (0.0, 1.0, 0.0, 1.0) }, //east (side)     // 9
    Vertex { position: ( 0.5, -0.5, 0.0), color: (0.0, 1.0, 0.0, 1.0) }, //east (side)     // 10
    Vertex { position: ( 0.5, -0.5, 1.0), color: (0.0, 1.0, 0.0, 1.0) }, //east (side)     // 11
    Vertex { position: ( 0.5,  0.5, 1.0), color: (0.0, 1.0, 0.0, 1.0) }, //east (side)     // 12

    Vertex { position: (-0.5, -0.5, 0.0), color: (0.0, 1.0, 0.0, 1.0) }, //west (side)     // 13
    Vertex { position: (-0.5,  0.5, 0.0), color: (0.0, 1.0, 0.0, 1.0) }, //west (side)     // 14
    Vertex { position: (-0.5,  0.5, 1.0), color: (0.0, 1.0, 0.0, 1.0) }, //west (side)     // 15
    Vertex { position: (-0.5, -0.5, 1.0), color: (0.0, 1.0, 0.0, 1.0) }, //west (side)     // 16

    Vertex { position: (-0.5,  0.5, 0.0), color: (0.0, 1.0, 1.0, 1.0) }, //top             // 17
    Vertex { position: (-0.5,  0.5, 1.0), color: (0.0, 1.0, 1.0, 1.0) }, //top             // 18
    Vertex { position: ( 0.5,  0.5, 1.0), color: (0.0, 1.0, 1.0, 1.0) }, //top             // 19
    Vertex { position: ( 0.5,  0.5, 0.0), color: (0.0, 1.0, 1.0, 1.0) }, //top             // 20

    Vertex { position: (-0.5, -0.5, 0.0), color: (0.0, 1.0, 1.0, 1.0) }, //bottom          // 21
    Vertex { position: (-0.5, -0.5, 1.0), color: (0.0, 1.0, 1.0, 1.0) }, //bottom          // 22
    Vertex { position: ( 0.5, -0.5, 1.0), color: (0.0, 1.0, 1.0, 1.0) }, //bottom          // 23
    Vertex { position: ( 0.5, -0.5, 0.0), color: (0.0, 1.0, 1.0, 1.0) }, //bottom          // 24
];

pub const INDICES: [u16; 36] = [
    1, 2, 3,
    1, 3, 4,   // south
    5, 6, 7,
    5, 7, 8,
    9, 10, 11,
    9, 11, 12,
    13, 14, 15,
    13, 15, 16, 
    17, 18, 19,
    17, 19, 20,
    21, 22, 23,
    21, 23, 24,
]; 

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32, f32)
}

implement_vertex!(Normal, normal);

pub const _FACE_NORMALS: [Normal; 6] = [
    Normal { normal: _SOUTH },
    Normal { normal: _NORTH },
    Normal { normal: _EAST },
    Normal { normal: _WEST },
    Normal { normal: _UP },
    Normal { normal: _DOWN },
];


