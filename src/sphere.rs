use vector::{Ray, Vec3};

use scene::{Intersector, Scene};

pub struct Sphere {
    pub c: Vec3,
    pub r: f64,
    pub col: Vec3,
}

impl Intersector for Sphere {
    fn intersect(&self, ray: &Ray, scene: &Scene, full_tracing: bool) -> (f64, Vec3) {
        let discriminant = ray.dir.dot(ray.origin - self.c).powf(2.0) - (ray.origin - self.c).len2()
            + self.r * self.r;

        if discriminant < 0.0 {
            return (-1.0, Vec3 { v: [0.0, 0.0, 0.0] });
        }

        let d1 = -ray.dir.dot(ray.origin - self.c) - discriminant.sqrt();
        let d2 = -ray.dir.dot(ray.origin - self.c) + discriminant.sqrt();
        let pos;
        let dist;

        if d1 > 0.0 {
            dist = d1;
        } else {
            dist = d2;
        }

        if dist > 0.0 {
            let mut stat = Vec3::default_vec();
            if full_tracing
            {
                pos = ray.origin + ray.dir * dist;
                let mut normal = pos - self.c;
                normal.normalize();

                stat = scene.get_static_light(pos, normal, ray.dir, self.col);
                scene.get_dynamic_light(pos, normal, ray.dir);
            }
            return (dist, stat);
        }

        (-1.0, Vec3::default_vec())
    }
}
