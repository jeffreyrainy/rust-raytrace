use material::Material;
use std::ptr;
use vector::{Ray, Vec3};

pub trait Intersector {
    //returns the distance and the color component
    fn intersect(&self, level: i64, ray: &Ray, scene: &Scene, full_tracing: bool) -> (f64, Vec3);
    fn id(&self) -> i64;
    fn set_id(&mut self, i64);
}

pub struct Scene {
    objects: Vec<Box<dyn Intersector>>,
    next_id: i64,
}

// todo: currently we have full shadow if object present in ray to light
// todo: we also have self-shadowing, for transparent spheres, for example

impl Scene {
    pub fn add(&mut self, mut object: Box<dyn Intersector>) {
        object.set_id(self.next_id);
        self.next_id = self.next_id + 1;
        self.objects.push(object);
    }
    //returns the distance and the color component
    pub fn intersect(
        &self,
        level: i64,
        ray: &Ray,
        full_tracing: bool,
        except: i64,
    ) -> (f64, Vec3, *const Box<dyn Intersector>) {

        let mut ret = (-1.0, Vec3 { v: [0., 0., 0.] }, ptr::null());

        if level>4
        {
            return ret;
        }

        let mut best_valid = false;
        let mut best_dist = 0.0;

        for object in &self.objects {
            let dist = object.intersect(level, &ray, self, full_tracing);
            // todo: 0.01 is hackish. intends to allows the further side of an object to still hit
            if object.id() != except || dist.0 > 0.01 {

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
        level: i64,
        pos: Vec3,
        normal: Vec3,
        ray_dir: Vec3,
        mat: Material,
        source: i64,
    ) -> Vec3 {
        let light_pos = Vec3 { v: [4.2, 6.0, 6.4] };

        let mut light_dir = light_pos - pos;
        light_dir.normalize();

        let mut diffuse = mat.diffuse * normal.dot(light_dir);
        if diffuse < 0.0 {
            diffuse = 0.0;
        }

        let mut reflected = ray_dir - 2.0 * (normal * ray_dir.dot(normal));
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
        let light_intersect = self.intersect(level, &ray, false, source);

        //todo: this is incorrect. if there's an object further away from the light, it should not cast shadow
        if light_intersect.0 > 0.0 {
            total = 0.0;
        }

        mat.col * total
    }

    pub fn get_dynamic_light(
        &self,
        level: i64,
        pos: Vec3,
        normal: Vec3,
        ray_dir: Vec3,
        mat: Material,
        source: i64,
    ) -> Vec3 {
        let mut normal_component = normal * ray_dir.dot(normal);
        let mut perpendicular_component = ray_dir - normal_component;

        let mut reflected = ray_dir - 2.0 * normal_component;
        reflected.normalize();

        normal_component.normalize();
        perpendicular_component.normalize();

        let reflected_ray = Ray {
            origin: pos,
            dir: reflected,
        };
        let reflect_intersect = self.intersect(level, &reflected_ray, true, source).1 * mat.specular;

        let incident_angle;
        let refracted_angle;
        let refracted_dir;

        if ray_dir.dot(normal) < 0.0 {
            incident_angle = ray_dir.dot(-1.0 * normal).acos();
            refracted_angle = (incident_angle.sin() / mat.ref_index).asin();

            refracted_dir = refracted_angle.sin() * perpendicular_component
                + refracted_angle.cos() * normal_component;

        } else {
            incident_angle = ray_dir.dot(normal).acos();
            let sin_angle = incident_angle.sin() * mat.ref_index;

            if sin_angle < 1.0 && sin_angle > -1.0 {
                refracted_angle = sin_angle.asin();
                refracted_dir = refracted_angle.sin() * perpendicular_component
                    + refracted_angle.cos() * normal_component;
            } else {
                refracted_dir = reflected;
            }
        }

        let refracted_ray = Ray {
            origin: pos,
            dir: refracted_dir,
        };
        let refracted_intersect = self.intersect(level, &refracted_ray, true, source).1 * mat.refractive;

        return reflect_intersect + refracted_intersect;
    }

    pub fn default_scene() -> Scene {
        Scene {
            objects: vec![],
            next_id: 1,
        }
    }
}
