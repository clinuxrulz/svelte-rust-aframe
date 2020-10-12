use crate::app::Sketch;
use crate::sketch::HParam;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Copy)]
pub struct Quaternion {
    pub w: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

#[derive(Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Vector4 {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Clone, Copy)]
pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Quaternion {
    pub fn new(w: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Quaternion { w, vx, vy, vz }
    }

    pub fn from_hparams(sk: &mut Sketch, w: HParam, vx: HParam, vy: HParam, vz: HParam) -> Self {
        unimplemented!();
    }

    pub fn from_uv(u: Vector, v: Vector) -> Self {
        unimplemented!();
    }

    pub fn from_axis_angle(axis: Vector, dtheta: f64) -> Self {
        unimplemented!();
    }

    pub fn magnitude(&self) -> f64 {
        (self.w * self.w + self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt()
    }

    pub fn with_magnitude(&self, s: f64) -> Self {
        *self * (s / self.magnitude())
    }

    pub fn rotation_u(&self) -> Vector {
        Vector {
            x: self.w * self.w + self.vx * self.vx - self.vy * self.vy - self.vz * self.vz,
            y: 2.0 * self.w * self.vz + 2.0 * self.vx * self.vy,
            z: 2.0 * self.vx * self.vz - 2.0 * self.w * self.vy,
        }
    }

    pub fn rotation_v(&self) -> Vector {
        Vector {
            x: 2.0 * self.vx * self.vy - 2.0 * self.w * self.vz,
            y: self.w * self.w - self.vx * self.vx + self.vy * self.vy - self.vz * self.vz,
            z: 2.0 * self.w * self.vx + 2.0 * self.vy * self.vz,
        }
    }

    pub fn rotation_n(&self) -> Vector {
        Vector {
            x: 2.0 * self.w * self.vy + 2.0 * self.vx * self.vz,
            y: 2.0 * self.vy * self.vz - 2.0 * self.w * self.vx,
            z: self.w * self.w - self.vx * self.vx - self.vy * self.vy + self.vz * self.vz,
        }
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Quaternion {
            w: self.w + rhs.w,
            vx: self.vx + rhs.vx,
            vy: self.vy + rhs.vy,
            vz: self.vz + rhs.vz,
        }
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Quaternion {
            w: self.w - rhs.w,
            vx: self.vx - rhs.vx,
            vy: self.vy - rhs.vy,
            vz: self.vz - rhs.vz,
        }
    }
}

impl Mul<f64> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Quaternion {
            w: self.w * rhs,
            vx: self.vx * rhs,
            vy: self.vy * rhs,
            vz: self.vz * rhs,
        }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }
}
