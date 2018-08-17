use std::ops::{Add, Div, Sub};

#[derive(Clone, Debug, Copy)]
pub struct Vec3 {
    pub v: [f64; 3],
}

impl Vec3 {
    pub fn normalize(&mut self) {
        let mut l = self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2];

        l = l.sqrt();
        if l > 0.0 {
            self.v[0] = self.v[0] / l;
            self.v[1] = self.v[1] / l;
            self.v[2] = self.v[2] / l;
        }
    }

    pub fn len2(&self) -> f64 {
        self.v[0] * self.v[0] + self.v[1] * self.v[1] + self.v[2] * self.v[2]
    }

    pub fn dot(self, other: Vec3) -> f64 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2]
    }

    pub fn default_vec() -> Vec3 {
        Vec3 { v: [0., 0., 0.] }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] + other.v[0],
                self.v[1] + other.v[1],
                self.v[2] + other.v[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            v: [
                self.v[0] - other.v[0],
                self.v[1] - other.v[1],
                self.v[2] - other.v[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3 {
            v: [self.v[0] / other, self.v[1] / other, self.v[2] / other],
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn normalize(&mut self) {
        self.dir.normalize();
    }

    pub fn default_ray() -> Ray {
        Ray {
            origin: Vec3::default_vec(),
            dir: Vec3::default_vec(),
        }
    }
}
