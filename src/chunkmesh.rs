extern crate glium;
extern crate vecmath;

use crate::cubemesh::*;

const CHUNK_SIZE: usize = 32;
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
        // cast to signed ints to avoid underflow
        let (x, y, z) = (x as i32, y as i32, z as i32);

        // east face
        if self.get_voxel(x+1, y, z) == 0 {
            self.push_vertex(EAST, matrix);
        }    

        // west face
        if self.get_voxel(x-1, y, z) == 0 {
            self.push_vertex(WEST, matrix);
        }    

        // top face
        if self.get_voxel(x, y+1, z) == 0 {
            self.push_vertex(TOP, matrix);
        }    

        //bottom face
        if self.get_voxel(x, y-1, z) == 0 {
            self.push_vertex(BOTTOM, matrix);
        }    

        // north face (pls don't sue me)
        if self.get_voxel(x, y, z+1) == 0 {
            self.push_vertex(NORTH, matrix);
        }    

        //south face
        if self.get_voxel(x, y, z-1) == 0 {
            self.push_vertex(SOUTH, matrix);
        }    
    }

    fn push_vertex (&mut self, face: [Vertex;4], matrix: vecmath::Matrix4<f32>) {
        for vertex in face {
            let mut vertex = vertex.clone();
            vertex.transform(matrix);
            self.vertices.push(vertex);
        }
        self.push_indices();
    }

    fn push_indices(&mut self) {
        for _i in 0..6 {
            let last_index = self.get_last_index();
            for index in INDICES {
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

    // this is a safe function that protects against out of bounds errors
    fn get_voxel(&self, x: i32, y: i32, z: i32) -> i8 {
        if x < 0 || y < 0 || z < 0 {
            return 0;
        }
        if x > 31 || y > 31 || z > 31 {
            return 0;
        }
        self.voxel_map[x as usize][y as usize][z as usize]
    }
}

