use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Color(Vec3);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub fn r(&self) -> f32 {
        self.0.x()
    }

    pub fn g(&self) -> f32 {
        self.0.y()
    }

    pub fn b(&self) -> f32 {
        self.0.z()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}",
               (255.0 * self.r()) as i32,
               (255.0 * self.g()) as i32,
               (255.0 * self.b()) as i32,
        )
    }
}
