use crate::ray_tracer::interface::object_base::HitRecord;
use crate::ray_tracer::{interface::material_base::Material, utils::Ray};
use crate::utils::vec4::{Color, Point};

pub struct Lambertian {
    albedo: Color, // the diffused color of the material
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn generate_reflected_ray(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let random_unit = Point::random_in_unit_sphere(); // picking point on a sphere
        let new_dir = random_unit.normalise() + hit_record.normal;

        let new_ray = Ray::new(
            hit_record.point_of_intersection,
            if new_dir.is_degenerate() {
                ray.direction
            } else {
                new_dir.normalise()
            },
        );

        // TODO: copies vec4 here (can we avoid this?)
        Some((self.albedo, new_ray))
    }
}
