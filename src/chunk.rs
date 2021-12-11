mod cubemesh;
extern crate noise;

pub const CHUNK_SIZE_X: f32 = 32;
pub const CHUNK_SIZE_Y: f32 = 32;
pub const CHUNK_SIZE_Z: f32 = 32;

pub struct Chunk {
    heightmap: Option<[f32; 16]>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            heightmap: None,
        }
    }

}
