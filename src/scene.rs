use vector::{Ray, Vec3};

pub trait Intersector {
    fn intersect(&self, ray: &Ray, scene: &Scene) -> (f64, Vec3);
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
            let det = object.intersect(&ray, self);

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

    pub fn get_static_light(&self, _pos: Vec3, normal: Vec3, ray_dir: Vec3, color: Vec3) -> Vec3 {
        let light_dir = Vec3 {
            v: [0.42, 0.6, 0.64],
        };

        let mut diffuse = normal.dot(light_dir);
        if diffuse < 0.0 {
            diffuse = 0.0;
        }

        let mut reflected = ray_dir - (normal * ray_dir.dot(normal)) * 2.0;
        reflected.normalize();

        let mut specular = light_dir.dot(reflected);

        if specular > 0.0 {
            specular = specular.powf(50.0);
        } else {
            specular = 0.0;
        }

        let mut total = diffuse + specular;

        if total > 1.0 {
            total = 1.0;
        }

        color * total
    }

    pub fn get_dynamic_light(&self, _pos: Vec3, _normal: Vec3, _dir: Vec3) {}

    pub fn default_scene() -> Scene {
        Scene { objects: vec![] }
    }
}
