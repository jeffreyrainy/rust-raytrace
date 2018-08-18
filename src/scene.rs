use vector::{Ray, Vec3};

pub trait Intersector {
    fn intersect(&self, ray: &Ray) -> (f64, Vec3);
}

pub struct Sphere {
    pub c: Vec3,
    pub r: f64,
    pub col: Vec3,
}

impl Intersector for Sphere {
    fn intersect(&self, ray: &Ray) -> (f64, Vec3) {
        let discriminant = ray.dir.dot(ray.origin - self.c).powf(2.0) - (ray.origin - self.c).len2()
            + self.r * self.r;

        if discriminant < 0.0 {
            return (-1.0, Vec3 { v: [0.0, 0.0, 0.0] });
        }

        let d1 = -ray.dir.dot(ray.origin - self.c) - discriminant.sqrt();
        let d2 = -ray.dir.dot(ray.origin - self.c) + discriminant.sqrt();
        let mut pos = Vec3::default_vec();
        let mut dist = 0.0;

        if d1 > 0.0 {
            pos = ray.origin + ray.dir * d1;
            dist = d1;
        } else {
            pos = ray.origin + ray.dir * d2;
            dist = d2;
        }

        (dist, self.col)
    }
}

pub struct Scene {
    objects: Vec<Box<dyn Intersector>>,
}

impl Scene {
    pub fn add(&mut self, object: Box<dyn Intersector>) {
        self.objects.push(object);
    }
    pub fn intersect(&self, ray: &Ray) -> Vec3 {
        let mut ret = Vec3 { v: [0., 0., 0.] };

        let mut best_valid = false;
        let mut best_dist = 0.0;

        for object in &self.objects {
            let det = object.intersect(&ray);

            if det.0 >= 0.0 {
                if !best_valid || det.0 < best_dist {
                    best_valid = true;
                    best_dist = det.0;
                    ret = det.1;
                }
            }
        }

        ret
    }

    pub fn default_scene() -> Scene {
        Scene { objects: vec![] }
    }
}
