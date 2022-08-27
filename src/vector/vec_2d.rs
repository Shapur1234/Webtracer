use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::traits::VectorOperation;
use num_traits::real::Real;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Clone> Vec2D<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[allow(dead_code)]
impl Vec2D<f32> {
    pub fn new_rand() -> Vec2D<f32> {
        let mut rng = thread_rng();
        Vec2D {
            x: rng.gen(),
            y: rng.gen(),
        }
    }

    pub fn new_rand_between(min: f32, max: f32) -> Vec2D<f32> {
        let mut rng = thread_rng();
        Vec2D {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
        }
    }

    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Vec2D {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
        }
    }
}

impl<T: Real> VectorOperation<T> for Vec2D<T> {
    fn cross(&self, _other: &Self) -> Self {
        unimplemented!()
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y
    }

    fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> T {
        self.x.powi(2) + self.y.powi(2)
    }

    fn unit_vec(&self) -> Vec2D<T> {
        *self / self.length()
    }

    fn dist_between(&self, other: &Self) -> T {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Add

impl<T, O> Add<Vec2D<T>> for Vec2D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn add(self, other: Vec2D<T>) -> Self::Output {
        Vec2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T, O> Add<T> for Vec2D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn add(self, other: T) -> Self::Output {
        Vec2D {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// AddAssign

impl<T> AddAssign<Vec2D<T>> for Vec2D<T>
where
    T: Add<T, Output = T> + Copy + Clone,
{
    fn add_assign(&mut self, other: Vec2D<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T> AddAssign<T> for Vec2D<T>
where
    T: Add<T, Output = T> + Copy + Clone,
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
    }
}

// Div

impl<T, O> Div<T> for Vec2D<T>
where
    T: Div<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn div(self, other: T) -> Self::Output {
        Vec2D {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// DivAssign

impl<T> DivAssign<T> for Vec2D<T>
where
    T: Div<T, Output = T> + Copy + Clone,
{
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}

// Mul

impl<T, O> Mul<T> for Vec2D<T>
where
    T: Mul<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn mul(self, other: T) -> Self::Output {
        Vec2D {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// MulAssign

impl<T> MulAssign<T> for Vec2D<T>
where
    T: Mul<T, Output = T> + Copy + Clone,
{
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
}

// Neg

impl<T, O> Neg for Vec2D<T>
where
    T: Neg<Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

// Sub

impl<T, O> Sub<Vec2D<T>> for Vec2D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn sub(self, other: Vec2D<T>) -> Self::Output {
        Vec2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T, O> Sub<T> for Vec2D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vec2D<O>;
    fn sub(self, other: T) -> Self::Output {
        Vec2D {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

// SubAssign

impl<T> SubAssign<Vec2D<T>> for Vec2D<T>
where
    T: Sub<T, Output = T> + Copy + Clone,
{
    fn sub_assign(&mut self, other: Vec2D<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<T> SubAssign<T> for Vec2D<T>
where
    T: Sub<T, Output = T> + Copy + Clone,
{
    fn sub_assign(&mut self, other: T) {
        self.x = self.x - other;
        self.y = self.y - other;
    }
}

// --------------------------------------------------
