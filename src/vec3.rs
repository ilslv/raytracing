use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign, Mul, Add, SubAssign, Sub, Div};
use std::slice::{Iter, IterMut};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Vec3 {
    vec: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { vec: [x, y, z] }
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

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.iter().map(|a| a * a).sum()
    }

    pub fn iter(&self) -> Iter<f32> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<f32> {
        self.vec.iter_mut()
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.iter()
            .zip(rhs)
            .map(|(a, b)| *a * *b)
            .sum()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            vec: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ]
        }
    }

    pub fn unit_vec(&self) -> Vec3 {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { vec: [-self.x(), -self.y(), -self.z()] }
    }
}

impl Index<u8> for Vec3 {
    type Output = f32;

    fn index(&self, index: u8) -> &Self::Output {
        &self.vec[index as usize]
    }
}

impl IndexMut<u8> for Vec3 {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.vec[index as usize]
    }
}

impl<'a> IntoIterator for &'a Vec3 {
    type Item = &'a f32;
    type IntoIter = Iter<'a, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Vec3 {
    type Item = &'a mut f32;
    type IntoIter = IterMut<'a, f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &'a Vec3) {
        for (a, b) in self.iter_mut().zip(rhs) {
            *a += *b;
        }
    }
}

impl<'a> SubAssign<&'a Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &'a Vec3) {
        for (a, b) in self.iter_mut().zip(rhs) {
            *a -= *b;
        }
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|a| *a *= rhs);
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.iter_mut().for_each(|a| *a /= rhs);
    }
}

impl<'a, 'b> Add<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'b Vec3) -> Self::Output {
        let mut result = Vec3::default();
        for ((r, a), b) in result.iter_mut().zip(self).zip(rhs) {
            *r = *a + *b;
        }
        result
    }
}

impl<'a, 'b> Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'b Vec3) -> Self::Output {
        let mut result = Vec3::default();
        for ((r, a), b) in result.iter_mut().zip(self).zip(rhs) {
            *r = *a - *b;
        }
        result
    }
}

impl<'a, 'b> Mul<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: &'b Vec3) -> Self::Output {
        let mut result = Vec3::default();
        for ((r, a), b) in result.iter_mut().zip(self).zip(rhs) {
            *r = *a * *b;
        }
        result
    }
}

impl<'a> Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut result = Vec3::default();
        for (r, a) in result.iter_mut().zip(self) {
            *r = *a * rhs;
        }
        result
    }
}

impl<'a> Mul<&'a Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Self::Output {
        let mut result = Vec3::default();
        for (r, a) in result.iter_mut().zip(rhs) {
            *r = *a * self;
        }
        result
    }
}

impl<'a> Div<f32> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        let mut result = Vec3::default();
        for (r, a) in result.iter_mut().zip(self) {
            *r = *a / rhs;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_product1() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);

        let c = a.cross(&b);
        assert_eq!(0.0, c.x());
        assert_eq!(0.0, c.y());
        assert_eq!(1.0, c.z());
    }

    #[test]
    fn cross_product2() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);

        let c = a.cross(&b);
        assert_eq!(-3.0, c.x());
        assert_eq!(6.0, c.y());
        assert_eq!(-3.0, c.z());
    }
}