use vector::{Ray, Vec3};

use scene::{Intersector, Scene};
use std::ptr;

pub struct Plane {
    pub pos: Vec3,
    pub norm: Vec3,
    pub col: Vec3,
}

impl Intersector for Plane {
    fn intersect(&self, ray: &Ray, scene: &Scene, full_tracing: bool) -> (f64, Vec3) {
        let denominator = ray.dir.dot(self.norm);

        if denominator == 0.0 {
            return (-1.0, Vec3::default_vec());
        } else {
            let distance = (self.pos - ray.origin).dot(self.norm) / ray.dir.dot(self.norm);
            let mut stat = Vec3::default_vec();

            if full_tracing
            {
                let pos = ray.origin + ray.dir * distance;

                stat = scene.get_static_light(pos, self.norm, ray.dir, self.col, ptr::null());
//todo                stat = scene.get_static_light(pos, self.norm, ray.dir, self.col, self);
                scene.get_dynamic_light(pos, self.norm, ray.dir);
            }

            return (distance, stat);
        }
    }
}
