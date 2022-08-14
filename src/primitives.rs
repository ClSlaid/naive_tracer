use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec3<Num: Add + Mul + Sub + Div>(Num, Num, Num);

pub type Rgb = Vec3<u16>;
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
impl Default for Rgb {
    fn default() -> Self {
        Self(0, 0, 0)
    }
}
