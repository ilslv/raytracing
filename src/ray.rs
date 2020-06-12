use crate::vec3::Vec3;
use crate::point3::Point3;

#[derive(Copy, Clone, Debug, Default)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}