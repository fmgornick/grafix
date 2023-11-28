use std::fs::File;
use std::io::prelude::*;

pub type Color = super::vector::Vec3<f64>;

impl Color {
    pub fn write(self, f: &mut File) -> std::io::Result<()> {
        writeln!(f, "{} {} {}", self.x, self.y, self.z)?;
        Ok(())
    }
}
