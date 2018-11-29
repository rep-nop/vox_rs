use crate::Vec3;
use std::collections::HashMap;
use three::Object;
use voxel::Voxel;

/// represents a 256x256x256 chunk of voxels
pub struct Chunk {
    pub group: three::Group,
    pub loc: Vec3<i32>,
    pub voxels: HashMap<Vec3<u8>, Voxel>,
}

impl Chunk {
    /// creates a new chunk
    pub fn new(factory: &mut three::Factory, loc: impl Into<Vec3<i32>>) -> Self {
        let group = factory.group();
        let voxels = HashMap::new();

        Chunk {
            group,
            loc: loc.into(),
            voxels,
        }
    }

    /// sets a voxel within the chunk,
    pub fn set_voxel(
        &mut self,
        factory: &mut three::Factory,
        loc: impl Into<Vec3<u8>>,
        color: u32,
    ) {
        let loc = loc.into();
        let vox = Voxel::new(factory, loc, color);
        self.voxels.insert(loc, vox);
    }

    /// adds the voxel groups to the scene
    pub fn add_to_scene(&self, scene: &mut three::Scene) {
        for (loc, voxel) in &self.voxels {
            scene.add(&voxel.group);
            voxel.group.set_position([
                (loc.x as i32 + self.loc.x * 256) as f32,
                (loc.y as i32 + self.loc.y * 256) as f32,
                (loc.z as i32 + self.loc.z * 256) as f32,
            ]);
        }
    }
}

pub struct ChunkBuilder<'a> {
    factory: &'a mut three::Factory,
    voxels: HashMap<Vec3<u8>, Voxel>,
    loc: Vec3<i32>,
}

impl<'a> ChunkBuilder<'a> {
    pub fn new(factory: &'a mut three::Factory, loc: impl Into<Vec3<i32>>) -> ChunkBuilder<'a> {
        ChunkBuilder {
            factory,
            voxels: HashMap::new(),
            loc: loc.into(),
        }
    }

    pub fn with_voxel(mut self, loc: impl Into<Vec3<u8>>, color: u32) -> ChunkBuilder<'a> {
        let loc = loc.into();
        self.voxels
            .insert(loc, Voxel::new(&mut self.factory, loc, color));

        self
    }

    pub fn finish(self) -> Chunk {
        Chunk {
            group: self.factory.group(),
            loc: self.loc,
            voxels: self.voxels,
        }
    }
}
