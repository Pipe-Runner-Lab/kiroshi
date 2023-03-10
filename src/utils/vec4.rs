use rand::prelude::*;
use std::{
    f32::consts::PI,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign},
};

const EPSILON: f32 = 1.0e-8;

#[derive(Clone, Copy)]
pub struct Vec4 {
    pub e: [f32; 4],
}

/// Color domain per channel => [0, 1]
pub type Color = Vec4;
pub type Point = Vec4;

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { e: [x, y, z, w] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn w(&self) -> f32 {
        self.e[3]
    }

    pub fn dot(&self, other: Self) -> f32 {
        self[0] * other[0] + self[1] * other[1] + self[2] * other[2] + self[3] * other[3]
    }

    pub fn length(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.e[1] * other.e[2] - self.e[2] * other.e[1],
            -(self.e[0] * other.e[2] - self.e[2] * other.e[0]),
            self.e[0] * other.e[1] - self.e[1] * other.e[0],
            0.,
        )
    }

    /// Normalises the vector in place
    pub fn normalised(&mut self) {
        *self /= self.length();
    }

    /// Returns a copy of the vector, normalized
    pub fn normalise(&self) -> Self {
        let new_point = *self; // this creates a copy (since we have Copy trait)
        new_point / self.length()
    }

    // TODO: Need a better mechanism for this + move somewhere else
    // ! Unclear about 255.99
    pub fn format_color(&self) -> String {
        format!(
            "{} {} {}",
            ((self[0].clamp(0., 0.999) * 256.) as u32),
            (self[1].clamp(0., 0.999) * 256.) as u32,
            (self[2].clamp(0., 0.999) * 256.) as u32
        )
    }

    // TODO: Does not account for w
    // TODO: Might give out zero vector
    pub fn random_in_unit_sphere() -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();
        let theta = (rng.gen::<f32>() - 0.5) * PI; // in radians (-pi/2, pi/2)
        let alpha = rng.gen::<f32>() * 2. * PI; // in radians (0, 2pi)

        Self {
            e: [r * alpha.cos(), r * theta.sin(), r * alpha.sin(), 0.],
        }
    }

    pub fn is_degenerate(&self) -> bool {
        self.length() < EPSILON
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

impl Add<Self> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        )
    }
}

impl AddAssign<Self> for Vec4 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
        self[3] += rhs[3];
    }
}

impl Sub<Self> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2],
            self[3] - rhs[3],
        )
    }
}

impl SubAssign<Self> for Vec4 {
    fn sub_assign(&mut self, rhs: Self) {
        self[0] -= rhs[0];
        self[1] -= rhs[1];
        self[2] -= rhs[2];
        self[3] -= rhs[3];
    }
}

// * Multiplying with another vector (We are doing a dot product here)
impl Mul<Self> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2],
            self[3] * rhs[3],
        )
    }
}

// * Multiplying with another vector (We are doing a dot product here)
impl MulAssign<Self> for Vec4 {
    fn mul_assign(&mut self, rhs: Self) {
        self[0] *= rhs[0];
        self[1] *= rhs[1];
        self[2] *= rhs[2];
        self[3] *= rhs[3];
    }
}

// * Multiplying with another vector (We are doing a scalar multiplication here)
impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self::new(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs,
            self[3] * rhs,
        )
    }
}

// * Multiplying with another vector (We are doing a scalar multiplication here)
impl MulAssign<f32> for Vec4 {
    fn mul_assign(&mut self, rhs: f32) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
        self[3] *= rhs;
    }
}

// * Multiplying with another vector (We are doing a scalar multiplication here)
impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        let rhs = if rhs == 0. {
            EPSILON
        } else {
            rhs
        };

        Self::new(
            self[0] / rhs,
            self[1] / rhs,
            self[2] / rhs,
            self[3] / rhs,
        )
    }
}

// * Multiplying with another vector (We are doing a scalar multiplication here)
impl DivAssign<f32> for Vec4 {
    fn div_assign(&mut self, rhs: f32) {
        let rhs = if rhs == 0. {
            EPSILON
        } else {
            rhs
        };


        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;
    }
}
