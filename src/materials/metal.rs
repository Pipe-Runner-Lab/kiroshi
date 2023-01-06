use crate::{
    ray_tracer::{
        interface::{material_base::Material, object_base::HitRecord},
        utils::Ray,
    },
    utils::vec4::{Color, Vec4},
};

pub struct Metal {
    albedo: Color,
    fuzz: Option<f32>,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Option<f32>) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn generate_reflected_ray(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let new_dir = ray.direction - hit_record.normal * ray.direction.dot(hit_record.normal) * 2.; // This will ideally be a unit vector as well according to the Maths

        let mut new_ray = Ray::new(
            hit_record.point_of_intersection,
            if new_dir.is_degenerate() {
                ray.direction
            } else {
                new_dir.normalise()
            },
        );

        if let Some(fuzz) = self.fuzz {
            let fuzzy_direction =
                (new_ray.direction + (Vec4::random_in_unit_sphere() * fuzz)).normalise();

            // if the fuzz direction calc leads ray into the surface
            if hit_record.normal.dot(fuzzy_direction) <= 0. {
                return None;
            }

            new_ray.direction = fuzzy_direction;
        }

        Some((
            self.albedo, // we are not using angle based attenuation here
            new_ray,
        ))
    }
}
