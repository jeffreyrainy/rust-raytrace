use vector::Vec3;

#[derive(Clone, Debug, Copy)]
pub struct Material {
    pub col: Vec3,
    pub diffuse: f64,
    pub specular: f64,
    pub refractive: f64,
}

impl Material{
    pub fn default_material() -> Material {
        Material {
            col: Vec3{v: [1.0, 1.0, 1.0]},
            diffuse: 0.8,
            specular: 0.2,
            refractive: 0.0,
        }
    }
}