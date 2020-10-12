pub struct Quaternion {
    pub w: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Vector4 {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point2d {
    pub x: f64,
    pub y: f64,
}

impl Quaternion {
    pub fn new(w: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Quaternion { w, vx, vy, vz }
    }
}
