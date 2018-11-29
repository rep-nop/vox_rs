// namespace
use crate::Vec3;
use three::Object;

/// represents a single voxel
pub struct Voxel {
    pub group: three::Group,
    pub loc: Vec3<u8>,
}

impl Voxel {
    /// creates a new voxel at given location with color
    pub fn new(factory: &mut three::Factory, loc: impl Into<Vec3<u8>>, color: u32) -> Self {
        let group = factory.group();
        let loc = loc.into();

        if color > 0xFFFFFF {
            panic!("invalid color!");
        }

        let mesh = {
            let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
            let material = three::material::Lambert {
                color: color,
                flat: false,
            };
            factory.mesh(geometry, material)
        };

        group.add(&mesh);
        group.set_position([loc.x as f32, loc.y as f32, loc.z as f32]);

        Voxel { group, loc }
    }
}
