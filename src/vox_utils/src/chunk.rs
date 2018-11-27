use voxel::Voxel;
use std::collections::HashMap;
use three::Object;

/// represents a 256x256x256 chunk of voxels
pub struct Chunk {
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

    /// adds the voxel groups to the scene
    pub fn add_to_scene(&self, scene: &mut three::Scene) {
        for (loc, voxel) in &self.voxels {
            scene.add(&voxel.group);
            voxel.group.set_position(
                [
                    (loc.0 as i32 + self.loc.0 * 256) as f32,
                    (loc.1 as i32 + self.loc.1 * 256) as f32,
                    (loc.2 as i32 + self.loc.2 * 256) as f32,
                ],
            );
        }
    }

}