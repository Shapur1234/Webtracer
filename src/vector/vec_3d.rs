use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::traits::VectorOperation;
use num_traits::real::Real;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy + Clone> Vec3D<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

#[allow(dead_code)]
impl Vec3D<f32> {
    pub fn new_rand() -> Vec3D<f32> {
        let mut rng = thread_rng();
        Vec3D {
            x: rng.gen(),
            y: rng.gen(),
            z: rng.gen(),
        }
    }

    pub fn new_rand_between(min: f32, max: f32) -> Vec3D<f32> {
        let mut rng = thread_rng();
        Vec3D {
            x: rng.gen_range(min..=max),
            y: rng.gen_range(min..=max),
            z: rng.gen_range(min..=max),
        }
    }

    pub fn new_rand_in_unit_sphere() -> Vec3D<f32> {
        loop {
            let vec = Vec3D::new_rand_between(-1.0, 1.0);
            if vec.length_squared() <= 1.0 {
                return vec;
            }
        }
    }

    pub fn clamp(&self, min: f32, max: f32) -> Self {
        Vec3D {
            x: self.x.clamp(min, max),
            y: self.y.clamp(min, max),
            z: self.z.clamp(min, max),
        }
    }

    pub fn rotate_around_x(&self, theta: f32) -> Vec3D<f32> {
        let rot_old = self;
        Vec3D::new(
            rot_old.x,
            rot_old.y * theta.cos() - rot_old.z * theta.sin(),
            rot_old.y * theta.sin() + rot_old.z * theta.cos(),
        )
    }

    pub fn rotate_around_y(&self, theta: f32) -> Vec3D<f32> {
        let rot_old = self;
        Vec3D::new(
            rot_old.x * theta.cos() + rot_old.z * theta.sin(),
            rot_old.y,
            -rot_old.x * theta.sin() + rot_old.z * theta.cos(),
        )
    }

    pub fn rotate_around_z(&self, theta: f32) -> Vec3D<f32> {
        let rot_old = self;
        Vec3D::new(
            rot_old.x * theta.cos() - rot_old.y * theta.sin(),
            rot_old.x * theta.sin() + rot_old.y * theta.cos(),
            rot_old.z,
        )
    }
}

impl<T: Real> VectorOperation<T> for Vec3D<T> {
    fn cross(&self, other: &Self) -> Self {
        Vec3D::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    fn length_squared(&self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    fn unit_vec(&self) -> Self {
        *self / self.length()
    }

    fn dist_between(&self, other: &Self) -> T {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }
}

// Add

impl<T, O> Add<Vec3D<T>> for Vec3D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn add(self, other: Vec3D<T>) -> Self::Output {
        Vec3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T, O> Add<T> for Vec3D<T>
where
    T: Add<T, Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn add(self, other: T) -> Self::Output {
        Vec3D {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

// AddAssign

impl<T> AddAssign<Vec3D<T>> for Vec3D<T>
where
    T: Add<T, Output = T> + Copy + Clone,
{
    fn add_assign(&mut self, other: Vec3D<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<T> AddAssign<T> for Vec3D<T>
where
    T: Add<T, Output = T> + Copy + Clone,
{
    fn add_assign(&mut self, other: T) {
        self.x = self.x + other;
        self.y = self.y + other;
        self.z = self.z + other;
    }
}

// Div

impl<T, O> Div<T> for Vec3D<T>
where
    T: Div<T, Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn div(self, other: T) -> Self::Output {
        Vec3D {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

// DivAssign

impl<T> DivAssign<T> for Vec3D<T>
where
    T: Div<T, Output = T> + Copy + Clone,
{
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

// Mul

impl<T, O> Mul<T> for Vec3D<T>
where
    T: Mul<T, Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn mul(self, other: T) -> Self::Output {
        Vec3D {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

// MulAssign

impl<T> MulAssign<T> for Vec3D<T>
where
    T: Mul<T, Output = T> + Copy + Clone,
{
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

// Neg

impl<T, O> Neg for Vec3D<T>
where
    T: Neg<Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// Sub

impl<T, O> Sub<Vec3D<T>> for Vec3D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn sub(self, other: Vec3D<T>) -> Self::Output {
        Vec3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T, O> Sub<T> for Vec3D<T>
where
    T: Sub<T, Output = O> + Copy + Clone,
{
    type Output = Vec3D<O>;
    fn sub(self, other: T) -> Self::Output {
        Vec3D {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

// SubAssign

impl<T> SubAssign<Vec3D<T>> for Vec3D<T>
where
    T: Sub<T, Output = T> + Copy + Clone,
{
    fn sub_assign(&mut self, other: Vec3D<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<T> SubAssign<T> for Vec3D<T>
where
    T: Sub<T, Output = T> + Copy + Clone,
{
    fn sub_assign(&mut self, other: T) {
        self.x = self.x - other;
        self.y = self.y - other;
        self.z = self.z - other;
    }
}
