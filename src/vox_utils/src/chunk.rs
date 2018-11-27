use voxel::Voxel;
use std::collections::HashMap;

/// represents a 256x256x256 chunk of voxels
struct Chunk {
    pub group: three::Group,
    pub loc: (i32, i32, i32),
    pub voxels: HashMap<(u8, u8, u8), Voxel>,
}

impl Chunk {
    /// creates a new chunk
    pub fn new(factory: &mut three::Factory, loc: (i32, i32, i32)) -> Self {
        let group = factory.group();
        let voxels: HashMap<(u8, u8, u8), Voxel> = HashMap::new();

        Chunk { group, loc, voxels }
    }
    /// sets a voxel within the chunk,
    pub fn set_voxel(&mut self, factory: &mut three::Factory, loc: (u8, u8, u8), color: u32) {
        let vox = Voxel::new(factory, loc, color);
        self.voxels.insert(loc, vox);
    }
}