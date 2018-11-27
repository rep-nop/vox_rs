// namespace
use three::Object;

/// represents a single voxel
pub struct Voxel {
    pub group: three::Group,
    pub loc: (u8, u8, u8),
}

impl Voxel {
    /// creates a new voxel at given location with color
    pub fn new(factory: &mut three::Factory, loc: (u8, u8, u8), color: u32) -> Self {
        let group = factory.group();

        if color > 0xFFFFFF {
            panic!("invalid color!");
        }

        let mesh = {
            let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
            let material = three::material::Lambert {
                color: 0xF8D790,
                flat: false,
            };
            factory.mesh(geometry, material)
        };

        group.add(&mesh);
        group.set_position([loc.0 as f32, loc.1 as f32, loc.2 as f32]);

        Voxel { group, loc }
    }
}