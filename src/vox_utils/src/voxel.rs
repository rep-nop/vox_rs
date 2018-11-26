use three::Object;

/// represents a single voxel
pub struct Voxel {
    pub group: three::Group,
    pub loc: (f32, f32, f32),
}

impl Voxel {
    /// creates a new voxel at given location with color
    pub fn new(factory: &mut three::Factory, loc: (f32, f32, f32), color: u32) -> Self {
        let group = factory.group();

        if color > 0xFFFFFF {
            panic!("invalid color!");
        }

        let mesh = factory.mesh(
            three::Geometry::cuboid(1.0, 1.0, 1.0),
            three::material::Basic {
                color: color,
                map: None,
            },
        );

        group.add(&mesh);

        Voxel { group, loc }
    }
}