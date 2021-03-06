use crate::vec3::Vec3;
use std::ops::{Index, IndexMut, Add, Sub, AddAssign, SubAssign};
use std::slice::{Iter, IterMut};

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Point3(pub Vec3);

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3(Vec3::new(x, y, z))
    }

    pub fn x(&self) -> f32 {
        self[0]
    }

    pub fn y(&self) -> f32 {
        self[1]
    }

    pub fn z(&self) -> f32 {
        self[2]
    }

    pub fn iter(&self) -> Iter<f32> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<f32> {
        self.0.iter_mut()
    }
}

impl Index<u8> for Point3 {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<u8> for Point3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3(self.0 + rhs)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point3(self.0 - rhs)
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs;
    }
}

impl SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.0 -= rhs
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Self {
        Point3(v)
    }
}