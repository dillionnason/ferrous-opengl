use glium::implement_vertex;

extern crate glium;
extern crate vecmath;

const CHUNK_SIZE: usize = 32;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: (f32, f32, f32),
}

implement_vertex!(Vertex, position);

const _N: [Vertex;4] = [
    Vertex { position: (-1.0, -1.0, 0.0) },
    Vertex { position: (-1.0,  1.0, 0.0) },
    Vertex { position: ( 1.0,  1.0, 0.0) },
    Vertex { position: ( 1.0, -1.0, 0.0) },
];

const _SOUTH: [i8; 12] = [-1, -1, 0, -1, 1, 0,  1, 1, 0,  1, -1, 0,];
const _NORTH: [i8; 12] = [ 1, -1, 0,  1, 1, 0, -1, 1, 0, -1, -1, 0,];

const _INDICES: [u8; 6] = [1, 2, 3, 1, 3, 4];

pub struct ChunkMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u8>, 
    voxel_map: [[u8; CHUNK_SIZE]; CHUNK_SIZE],
}

impl Default for ChunkMesh {
    fn default() -> ChunkMesh {
        let vox: [[u8; CHUNK_SIZE]; CHUNK_SIZE] = [[1; CHUNK_SIZE]; CHUNK_SIZE];
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
        for x in self.voxel_map {
            for y in x {
                if y == 1 {
                    for vertex in _N {
                        self.vertices.push(vertex);
                    }
                    let last_index = self.get_last_index();
                    for index in _INDICES {
                        self.indices.push(index + last_index);
                    }
                }
            }
        }
    }

    // clone the last index to a new value
    fn get_last_index(&self) -> u8 {
        let last_index: u8 = 0;
        if !self.indices.is_empty() {
            let last_index = self.indices.last().unwrap();
            let last_index = last_index.clone();
        }
        last_index
    }
}

