use crate::{
    ray_tracer::{
        interface::{material_base::Material, object_base::HitRecord},
        utils::Ray,
    },
    utils::vec4::Color,
};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn generate_reflected_ray(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let new_dir =
            ray.direction - hit_record.normal * ray.direction.dot(hit_record.normal) * 2.; // This will ideally be a unit vector as well according to the Maths

        let new_ray = Ray::new(
            hit_record.point_of_intersection,
            if new_dir.is_degenerate() {
                ray.direction
            } else {
                new_dir.normalise()
            },
        );
        Some((
            self.albedo, // we are not using angle based attenuation here
            new_ray,
        ))
    }
}
