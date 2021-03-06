use material::Material;
use vector::{Ray, Vec3};

use scene::{Intersector, Scene};

pub struct Sphere {
    pub center: Vec3,
    pub r: f64,
    pub mat: Material,
    pub id: i64,
}

impl Intersector for Sphere {
    fn intersect(&self, level: i64, ray: &Ray, scene: &Scene, full_tracing: bool) -> (f64, Vec3) {
        let discriminant = ray.dir.dot(ray.origin - self.center).powf(2.0)
            - (ray.origin - self.center).len2() + self.r * self.r;

        if discriminant < 0.0 {
            return (-1.0, Vec3 { v: [0.0, 0.0, 0.0] });
        }

        let d1 = -ray.dir.dot(ray.origin - self.center) - discriminant.sqrt();
        let d2 = -ray.dir.dot(ray.origin - self.center) + discriminant.sqrt();
        let pos;
        let dist;

        // todo: 0.01 is hackish. intends to allows the further side of an object to still hit
        if d1 > 0.01 {
            dist = d1;
        } else {
            dist = d2;
        }

        if dist > 0.01 {
            let mut stat = Vec3::default_vec();
            let mut dyn = Vec3::default_vec();

            if full_tracing {
                pos = ray.origin + ray.dir * dist;
                let mut norm = pos - self.center;
                norm.normalize();

                stat = scene.get_static_light(level + 1, pos, norm, ray.dir, self.mat, self.id());
                dyn = scene.get_dynamic_light(level + 1, pos, norm, ray.dir, self.mat, self.id());
            }
            return (dist, stat + dyn);
        }

        (-1.0, Vec3::default_vec())
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = id
    }
}
