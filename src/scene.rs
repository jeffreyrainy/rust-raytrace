use vector::{Ray, Vec3};

trait Intersector {
    fn intersect(&self, ray: &Ray) -> (f64, Vec3);
}

pub struct Sphere {
    pub c: Vec3,
    pub r: f64,
    pub col: Vec3,
}

impl Intersector for Sphere {
    fn intersect(&self, ray: &Ray) -> (f64, Vec3) {
        (
            ray.dir.dot(ray.origin - self.c).powf(2.0) - (ray.origin - self.c).len2()
                + self.r * self.r,
            self.col,
        )
    }
}

pub struct Scene {
    objects: Vec<Box<dyn Intersector>>,
}

impl Scene {
    pub fn intersect(&self, ray: &Ray) -> Vec3 {
        let mut ret = Vec3 { v: [0., 0., 0.] };

        let mut bestValid = false;
        let mut bestDist = 0.0;

        for object in &self.objects {
            let det = object.intersect(&ray);

            if det.0 >= 0.0 {
                if (!bestValid || det.0 < bestDist) {
                    bestValid = true;
                    bestDist = det.0;
                    ret = det.1;
                }
            }
        }

        ret
    }

    pub fn default_scene() -> Scene {
        Scene {
            objects: vec![
                Box::new(Sphere {
                    c: Vec3 { v: [0., 0., -1.] },
                    r: 0.3,
                    col: Vec3 { v: [0.8, 0.7, 0.1] },
                }),
                Box::new(Sphere {
                    c: Vec3 { v: [0., 0.4, -1.] },
                    r: 0.3,
                    col: Vec3 { v: [0.5, 0.6, 0.9] },
                }),
            ],
        }
    }
}
