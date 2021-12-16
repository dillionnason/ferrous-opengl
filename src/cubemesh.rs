use glium::implement_vertex;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 4],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn transform(&mut self, mat: [[f32; 4]; 4 ]) {
        self.position = vecmath::col_mat4_transform(mat, self.position);
    }
}

pub const NORTH: [Vertex;4] = [
    Vertex { position: [1.0, 0.0, 1.0, 1.0], tex_coords: [1.0, 0.0]},  
    Vertex { position: [1.0, 1.0, 1.0, 1.0], tex_coords: [1.0, 1.0]}, 
    Vertex { position: [0.0, 1.0, 1.0, 1.0], tex_coords: [0.0, 1.0]}, 
    Vertex { position: [0.0, 0.0, 1.0, 1.0], tex_coords: [0.0, 0.0]}, 
];
pub const SOUTH: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },    
    Vertex { position: [0.0, 1.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },    
    Vertex { position: [1.0, 1.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },    
    Vertex { position: [1.0, 0.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },  
];
pub const EAST: [Vertex;4] = [
    Vertex { position: [1.0, 1.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },      
    Vertex { position: [1.0, 0.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },      
    Vertex { position: [1.0, 0.0, 1.0, 1.0], tex_coords: [1.0, 0.0] },      
    Vertex { position: [1.0, 1.0, 1.0, 1.0], tex_coords: [1.0, 1.0] },   
];
pub const WEST: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },      
    Vertex { position: [0.0, 1.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },      
    Vertex { position: [0.0, 1.0, 1.0, 1.0], tex_coords: [0.0, 1.0] },      
    Vertex { position: [0.0, 0.0, 1.0, 1.0], tex_coords: [0.0, 0.0] },    
];
pub const TOP: [Vertex;4] = [
    Vertex { position: [0.0, 1.0, 0.0, 1.0], tex_coords: [0.0, 0.0] },              
    Vertex { position: [0.0, 1.0, 1.0, 1.0], tex_coords: [0.0, 1.0] },              
    Vertex { position: [1.0, 1.0, 1.0, 1.0], tex_coords: [1.0, 1.0] },              
    Vertex { position: [1.0, 1.0, 0.0, 1.0], tex_coords: [1.0, 0.0] },   
];
pub const BOTTOM: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 0.0, 1.0], tex_coords: [0.0, 1.0] },           
    Vertex { position: [0.0, 0.0, 1.0, 1.0], tex_coords: [0.0, 0.0] },           
    Vertex { position: [1.0, 0.0, 1.0, 1.0], tex_coords: [1.0, 0.0] },           
    Vertex { position: [1.0, 0.0, 0.0, 1.0], tex_coords: [1.0, 1.0] },   
];

pub const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];