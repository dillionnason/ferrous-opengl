use glium::implement_vertex;

extern crate glium;
extern crate vecmath;

const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 4],
}

implement_vertex!(Vertex, position);

impl Vertex {
    pub fn transform(&mut self, mat: [[f32; 4]; 4 ]) {
        self.position = vecmath::col_mat4_transform(mat, self.position);
    }
}

const _N: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 1.0, 1.0] },  
    Vertex { position: [0.0, 1.0, 1.0, 1.0] }, 
    Vertex { position: [1.0, 1.0, 1.0, 1.0] }, 
    Vertex { position: [1.0, 0.0, 1.0, 1.0] }, 
];
const _S: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 0.0, 1.0] },    
    Vertex { position: [0.0, 1.0, 0.0, 1.0] },    
    Vertex { position: [1.0, 1.0, 0.0, 1.0] },    
    Vertex { position: [1.0, 0.0, 0.0, 1.0] },  
];
const _E: [Vertex;4] = [
    Vertex { position: [1.0, 1.0, 0.0, 1.0] },      
    Vertex { position: [1.0, 0.0, 0.0, 1.0] },      
    Vertex { position: [1.0, 0.0, 1.0, 1.0] },      
    Vertex { position: [1.0, 1.0, 1.0, 1.0] },   
];
const _W: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 0.0, 1.0] },      
    Vertex { position: [0.0, 1.0, 0.0, 1.0] },      
    Vertex { position: [0.0, 1.0, 1.0, 1.0] },      
    Vertex { position: [0.0, 0.0, 1.0, 1.0] },    
];
const _T: [Vertex;4] = [
    Vertex { position: [0.0, 1.0, 0.0, 1.0] },              
    Vertex { position: [0.0, 1.0, 1.0, 1.0] },              
    Vertex { position: [1.0, 1.0, 1.0, 1.0] },              
    Vertex { position: [1.0, 1.0, 0.0, 1.0] },   
];
const _B: [Vertex;4] = [
    Vertex { position: [0.0, 0.0, 0.0, 1.0] },           
    Vertex { position: [0.0, 0.0, 1.0, 1.0] },           
    Vertex { position: [1.0, 0.0, 1.0, 1.0] },           
    Vertex { position: [1.0, 0.0, 0.0, 1.0] },   
];


const _INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>, 
    voxel_map: [[[i8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Default for ChunkMesh {
    fn default() -> ChunkMesh {
        let vox: [[[i8; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE] = [[[1; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
        ChunkMesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            voxel_map: vox, 
        }
    }
}

impl ChunkMesh {
    pub fn new() -> ChunkMesh {
        ChunkMesh {
           ..Default::default() 
        }
    }

    pub fn generate_mesh(&mut self) {        
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.voxel_map[x][y][z] != 0 {
                        let matrix = [
                            [ 1.0, 0.0, 0.0, 0.0 ],
                            [ 0.0, 1.0, 0.0, 0.0 ],
                            [ 0.0, 0.0, 1.0, 0.0 ],
                            [ x as f32 ,  y as f32, z as f32, 1.0f32 ]
                        ];
                        self.draw_cube(matrix, x, y, z);
                    }
                }
            }
        }
    }

    fn draw_cube(
        &mut self, 
        matrix: vecmath::Matrix4<f32>, 
        x: usize, 
        y: usize, 
        z: usize
    ) 
    {
        if z == 31 {
            for vertex in _N {
                self.push_vertex(vertex, matrix);
            }
            self.push_indices(); 
        }
        if z == 0 {
            for vertex in _S {
                self.push_vertex(vertex, matrix);
            }
            self.push_indices(); 
        }
        if y == 31 {
            for vertex in _T {
                self.push_vertex(vertex, matrix);
            }
            self.push_indices(); 
        }
        if y == 0 {
            for vertex in _B {
                self.push_vertex(vertex, matrix);
            }
            self.push_indices(); 
        }
        if x == 31 {
            for vertex in _E {
                self.push_vertex(vertex, matrix);
            }
            self.push_indices(); 
        }
        if  x == 0 {
            for vertex in _W {
                self.push_vertex(vertex, matrix);
            }
            self.push_indices(); 
        }
    }

    fn push_vertex (&mut self, vertex: Vertex, matrix: vecmath::Matrix4<f32>) {
        let mut vertex = vertex.clone();
        vertex.transform(matrix);
        self.vertices.push(vertex);
    }

    fn push_indices(&mut self) {
        for i in 0..6 {
            let last_index = self.get_last_index();
            for index in _INDICES {
                self.indices.push(index + last_index);
            }
        }
    }

    // clone the last index to a new value
    fn get_last_index(&self) -> u32 {
        if self.indices.is_empty() {
            let last_index: u32 = 0;
            return last_index;
        }
        let last_index = self.indices.last().unwrap();
        let mut last_index = last_index.clone();
        last_index += 1;
        last_index
    }
}

