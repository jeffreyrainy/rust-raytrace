use material::Material;
use vector::{Ray, Vec3};

use scene::{Intersector, Scene};

pub struct Plane {
    pub pos: Vec3,
    pub norm: Vec3,
    pub mat: Material,
    pub id: i64,
}

impl Intersector for Plane {
    fn intersect(&self, level: i64, ray: &Ray, scene: &Scene, full_tracing: bool) -> (f64, Vec3) {
        let denominator = ray.dir.dot(self.norm);

        if denominator == 0.0 {
            return (-1.0, Vec3::default_vec());
        } else {
            let distance = (self.pos - ray.origin).dot(self.norm) / ray.dir.dot(self.norm);
            let mut stat = Vec3::default_vec();
            let mut dyn = Vec3::default_vec();

            if full_tracing {
                let pos = ray.origin + ray.dir * distance;

                stat = scene.get_static_light(level + 1, pos, self.norm, ray.dir, self.mat, self.id());
                dyn = scene.get_dynamic_light(level + 1, pos, self.norm, ray.dir, self.mat, self.id());
            }

            return (distance, stat + dyn);
        }
    }

    fn id(&self) -> i64 {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = id
    }
}
