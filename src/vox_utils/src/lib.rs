extern crate three;

/// chunks yeet
pub mod chunk;
/// voxels yeet
pub mod voxel;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from((x, y, z): (T, T, T)) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

pub struct Color {
    pub a: u32,
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Color {
    pub fn new(alpha: impl Into<Option<u8>>, red: u8, green: u8, blue: u8) -> Color {
        Color {
            a: u32::from(alpha.into().unwrap_or(0x00)),
            r: u32::from(red),
            g: u32::from(green),
            b: u32::from(blue),
        }
    }

    pub fn into_u32(self) -> u32 {
        (self.a << 24) | (self.r << 16) | (self.g << 8) | self.b
    }
}
