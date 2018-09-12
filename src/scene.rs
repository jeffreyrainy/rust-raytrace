use std::ptr;
use vector::{Ray, Vec3};

pub trait Intersector {
    //returns the distance and the color component
    fn intersect(&self, ray: &Ray, scene: &Scene, full_tracing: bool) -> (f64, Vec3);
    fn id(&self) -> i64;
    fn set_id(&mut self, i64);
}

pub struct Scene {
    objects: Vec<Box<dyn Intersector>>,
    next_id: i64,
}

impl Scene {
    pub fn add(&mut self, mut object: Box<dyn Intersector>) {
        object.set_id(self.next_id);
        self.next_id = self.next_id + 1;
        self.objects.push(object);
    }
    //returns the distance and the color component
    pub fn intersect(
        &self,
        ray: &Ray,
        full_tracing: bool,
        except: i64,
    ) -> (f64, Vec3, *const Box<dyn Intersector>) {
        let mut ret = (-1.0, Vec3 { v: [0., 0., 0.] }, ptr::null());

        let mut best_valid = false;
        let mut best_dist = 0.0;

        for object in &self.objects {
            if object.id() != except {
                let dist = object.intersect(&ray, self, full_tracing);

                if dist.0 > 0.0 {
                    if !best_valid || dist.0 < best_dist {
                        best_valid = true;
                        best_dist = dist.0;
                        ret.0 = dist.0;
                        ret.1 = dist.1;
                        ret.2 = object;
                    }
                }
            }
        }

        ret
    }

    pub fn get_static_light(
        &self,
        pos: Vec3,
        normal: Vec3,
        ray_dir: Vec3,
        color: Vec3,
        source: i64,
    ) -> Vec3 {
        let light_pos = Vec3 { v: [4.2, 6.0, 6.4] };

        let mut light_dir = light_pos - pos;
        light_dir.normalize();

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

        let ray = Ray {
            origin: pos,
            dir: light_dir,
        };
        let light_intersect = self.intersect(&ray, false, source);

        //todo: this is incorrect. if there's an object further away from the light, it should not cast shadow
        if light_intersect.0 > 0.0 {
            total = 0.0;
        }

        color * total
    }

    pub fn get_dynamic_light(
        &self,
        pos: Vec3,
        normal: Vec3,
        ray_dir: Vec3,
        _color: Vec3,
        source: i64,
    ) -> Vec3 {
        let mut reflected = ray_dir - (normal * ray_dir.dot(normal)) * 2.0;
        reflected.normalize();

        let ray = Ray {
            origin: pos,
            dir: reflected,
        };
        let reflect_intersect = self.intersect(&ray, true, source);

        return reflect_intersect.1;
    }

    pub fn default_scene() -> Scene {
        Scene {
            objects: vec![],
            next_id: 1,
        }
    }
}
