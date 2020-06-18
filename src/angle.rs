use std::f32::consts::PI;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Degrees(pub f32);

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct Radians(pub f32);

impl Add for Degrees {
    type Output = Degrees;

    fn add(self, rhs: Self) -> Self::Output {
        Degrees(self.0 + rhs.0)
    }
}

impl AddAssign for Degrees {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Degrees {
    type Output = Degrees;

    fn sub(self, rhs: Self) -> Self::Output {
        Degrees(self.0 - rhs.0)
    }
}

impl SubAssign for Degrees {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Degrees {
    type Output = Degrees;

    fn mul(self, rhs: Self) -> Self::Output {
        Degrees(self.0 * rhs.0)
    }
}

impl MulAssign for Degrees {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Degrees {
    type Output = Degrees;

    fn div(self, rhs: Self) -> Self::Output {
        Degrees(self.0 / rhs.0)
    }
}

impl DivAssign for Degrees {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Add for Radians {
    type Output = Radians;

    fn add(self, rhs: Self) -> Self::Output {
        Radians(self.0 + rhs.0)
    }
}

impl AddAssign for Radians {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Radians {
    type Output = Radians;

    fn sub(self, rhs: Self) -> Self::Output {
        Radians(self.0 - rhs.0)
    }
}

impl SubAssign for Radians {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Mul for Radians {
    type Output = Radians;

    fn mul(self, rhs: Self) -> Self::Output {
        Radians(self.0 * rhs.0)
    }
}

impl MulAssign for Radians {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Radians {
    type Output = Radians;

    fn div(self, rhs: Self) -> Self::Output {
        Radians(self.0 / rhs.0)
    }
}

impl DivAssign for Radians {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl From<Degrees> for Radians {
    fn from(d: Degrees) -> Self {
        Radians(d.0 * PI / 180.0)
    }
}

impl From<Radians> for Degrees {
    fn from(r: Radians) -> Self {
        Degrees(r.0 * 180.0 / PI)
    }
}

impl From<f32> for Radians {
    fn from(f: f32) -> Self {
        Radians(f)
    }
}

impl From<f32> for Degrees {
    fn from(f: f32) -> Self {
        Degrees(f)
    }
}

impl From<Degrees> for f32 {
    fn from(d: Degrees) -> Self {
        d.0
    }
}

impl From<Radians> for f32 {
    fn from(r: Radians) -> Self {
        r.0
    }
}
