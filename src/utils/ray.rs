use super::vector::{Dir3, Point3};

#[allow(dead_code)]
pub struct Ray {
    origin: Point3,
    dir: Dir3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Dir3) -> Ray {
        Ray { origin, dir }
    }

    pub fn at(self, t: f64) -> Point3 {
        self.origin.clone() + self.dir * t
    }
}
