use crate::app::Sketch;
use crate::sketch::HParam;

use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;

const ANGLE_COS_EPS: f64 = 1e-6;
const LENGTH_EPS: f64 = 1e-6;
const VERY_POSITIVE: f64 = 1e10;
const VERY_NEGATIVE: f64 = -1e10;

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
        let n = u.cross(v);
        let q;
        let tr = 1.0 + u.x + v.y + n.z;
        if tr > 1e-4 {
            let s = 2.0 * tr.sqrt();
            q = Quaternion {
                w: s / 4.0,
                vx: (v.z - n.y) / s,
                vy: (n.x - u.z) / s,
                vz: (u.y - v.x) / s,
            };
        } else {
            if u.x > v.y && u.x > n.z {
                let s = 2.0 * (1.0 + u.x - v.y - n.z).sqrt();
                q = Quaternion {
                    w: (v.z - n.y) / s,
                    vx: s / 4.0,
                    vy: (u.y + v.x) / s,
                    vz: (n.x + u.z) / s,
                };
            } else if v.y > n.z {
                let s = 2.0 * (1.0 - u.x + v.y - n.z).sqrt();
                q = Quaternion {
                    w: (n.x - u.z) / s,
                    vx: (u.y + v.x) / s,
                    vy: s / 4.0,
                    vz: (v.z + n.y) / s,
                }
            } else {
                let s = 2.0 * (1.0 - u.x - v.y + n.z).sqrt();
                q = Quaternion {
                    w: (u.y - v.x) / s,
                    vx: (n.x + u.z) / s,
                    vy: (v.z + n.y) / s,
                    vz: s / 4.0,
                };
            }
        }

        return q.with_magnitude(1.0);
    }

    pub fn from_axis_angle(mut axis: Vector, dtheta: f64) -> Self {
        let c = (dtheta / 2.0).cos();
        let s = (dtheta / 2.0).sin();
        axis = axis.with_magnitude(s);
        Quaternion {
            w: c,
            vx: axis.x,
            vy: axis.y,
            vz: axis.z,
        }
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

    pub fn rotate(&self, p: Vector) -> Vector {
        self.rotation_u() * p.x + self.rotation_v() * p.y + self.rotation_n() * p.z
    }

    pub fn inverse(&self) -> Self {
        (Quaternion {
            w: self.w,
            vx: -self.vx,
            vy: -self.vy,
            vz: -self.vz,
        })
        .with_magnitude(1.0)
    }

    pub fn to_the(&self, p: f64) -> Self {
        if self.w >= 1.0f64 - 1e-6f64 {
            return Quaternion::new(1.0, 0.0, 0.0, 0.0);
        } else if self.w <= -1.0f64 + 1e-6f64 {
            return Quaternion::new(-1.0, 0.0, 0.0, 0.0);
        }

        let mut axis = Vector::new(self.vx, self.vy, self.vz);
        let mut theta = self.w.acos();
        theta *= p;
        axis = axis.with_magnitude(theta.sin());
        Quaternion {
            w: theta.cos(),
            vx: axis.x,
            vy: axis.y,
            vz: axis.z,
        }
    }

    pub fn mirror(&self) -> Self {
        let mut u = self.rotation_u();
        let mut v = self.rotation_v();
        u = u * -1.0f64;
        v = v * -1.0f64;
        Quaternion::from_uv(u, v)
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

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, b: Self) -> Self {
        let sa = self.w;
        let sb = b.w;
        let va = Vector::new(self.vx, self.vy, self.vz);
        let vb = Vector::new(b.vx, b.vy, b.vz);
        let vr = vb * sa + va * sb + va.cross(vb);
        Quaternion {
            w: sa * sb - va.dot(vb),
            vx: vr.x,
            vy: vr.y,
            vz: vr.z,
        }
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn from_hparams(x: HParam, y: HParam, z: HParam) -> Self {
        unimplemented!();
    }

    pub fn cross(&self, rhs: Self) -> Self {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.z,
        }
    }

    pub fn dot(&self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn direction_cosine_with(&self, mut b: Vector) -> f64 {
        let a = self.with_magnitude(1.0);
        b = b.with_magnitude(1.0);
        a.dot(b)
    }

    pub fn normal(&self, which: u32) -> Vector {
        let mut n: Vector;
        let xa = self.x.abs();
        let ya = self.y.abs();
        let za = self.z.abs();
        if self.equals_with_def_tol(Vector::new(0.0, 0.0, 1.0)) {
            n = Vector::new(1.0, 0.0, 0.0);
        } else if xa < ya && xa < za {
            n = Vector::new(0.0, self.z, -self.y);
        } else if ya < za {
            n = Vector::new(-self.z, 0.0, self.x);
        } else {
            n = Vector::new(self.y, -self.x, 0.0);
        }
        match which {
            0 => {}
            1 => {
                n = self.cross(n);
            }
            _ => {
                panic!("{}", "Unexpected vector normal index");
            }
        }
        n.with_magnitude(1.0)
    }

    pub fn rotated_about_origin_axis(&self, origin: Vector, axis: Vector, theta: f64) -> Vector {
        (*self - origin).rotated_about_axis(axis, theta) + origin
    }

    pub fn rotated_about_axis(&self, mut axis: Vector, theta: f64) -> Vector {
        let c = theta.cos();
        let s = theta.sin();
        axis = axis.with_magnitude(1.0);
        Vector::new(
            (self.x) * (c + (1.0 - c) * (axis.x) * (axis.x))
                + (self.y) * ((1.0 - c) * (axis.x) * (axis.y) - s * (axis.z))
                + (self.z) * ((1.0 - c) * (axis.x) * (axis.z) + s * (axis.y)),
            (self.x) * ((1.0 - c) * (axis.y) * (axis.x) + s * (axis.z))
                + (self.y) * (c + (1.0 - c) * (axis.y) * (axis.y))
                + (self.z) * ((1.0 - c) * (axis.y) * (axis.z) - s * (axis.x)),
            (self.x) * ((1.0 - c) * (axis.z) * (axis.x) - s * (axis.y))
                + (self.y) * ((1.0 - c) * (axis.z) * (axis.y) + s * (axis.x))
                + (self.z) * (c + (1.0 - c) * (axis.z) * (axis.z)),
        )
    }

    pub fn mag_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn with_magnitude(&self, v: f64) -> Vector {
        let m = self.magnitude();
        if m == 0.0f64 {
            if v.abs() > 1e-100 {
                panic!("Vector::with_magnatude(...) of zero vector!");
            }
            return Vector::new(0.0, 0.0, 0.0);
        } else {
            return *self * (v / m);
        }
    }

    pub fn equals(&self, rhs: Self, tol: f64) -> bool {
        let dv: Vector = *self - rhs;
        if dv.x.abs() > tol {
            return false;
        };
        if dv.y.abs() > tol {
            return false;
        };
        if dv.z.abs() > tol {
            return false;
        };
        return dv.mag_squared() < tol * tol;
    }

    pub fn equals_with_def_tol(&self, rhs: Self) -> bool {
        self.equals(rhs, LENGTH_EPS)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
