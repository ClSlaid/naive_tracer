use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Debug)]
pub struct Num3<Num: Add + Mul + Sub + Div + Debug + Display>(Num, Num, Num);

impl<N: Add + Mul + Sub + Div + Debug + Display> Num3<N> {
    pub fn nums_string(&self) -> String {
        format!("{} {} {}", self.0, self.1, self.2)
    }
}

pub type Rgb = Num3<u16>;
impl Rgb {
    pub fn new(r: u16, g: u16, b: u16) -> Self {
        Self(r, g, b)
    }
    pub fn r(&self) -> u16 {
        self.0
    }
    pub fn g(&self) -> u16 {
        self.1
    }
    pub fn b(&self) -> u16 {
        self.2
    }
}

pub type Vec3 = Num3<f64>;
pub type Point3 = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self(0.0, 0.0, 0.0)
    }
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub const fn x(&self) -> f64 {
        self.0
    }
    pub const fn y(&self) -> f64 {
        self.1
    }
    pub const fn z(&self) -> f64 {
        self.2
    }

    pub fn abs(&self) -> f64 {
        (self.x().powi(2) + self.y().powi(2) + self.z().powi(2)).sqrt()
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Self::new(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn unit_vec(&self) -> Vec3 {
        (*self) / self.abs()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Color(Vec3);
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        let m = r.abs().max(g.abs().max(b.abs()));
        if m <= 1.0 {
            Self(Vec3::new(r, g, b))
        } else {
            Self(Vec3::new(r / m, g / m, b / m))
        }
    }
}

impl From<Rgb> for Color {
    fn from(rgb: Rgb) -> Self {
        let x = (rgb.0 as f64) / 256.0;
        let y = (rgb.1 as f64) / 256.0;
        let z = (rgb.2 as f64) / 256.0;
        Self::new(x, y, z)
    }
}

impl From<Color> for Rgb {
    fn from(c: Color) -> Self {
        let max = u16::MAX as f64;
        Self::new(
            (c.0 .0 * max) as u16,
            (c.0 .1 * max) as u16,
            (c.0 .2 * max) as u16,
        )
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Self::new(v.0, v.1, v.2)
    }
}
