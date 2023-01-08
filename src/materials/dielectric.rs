use crate::{
    ray_tracer::{
        interface::{material_base::Material, object_base::HitRecord},
        utils::Ray,
    },
    utils::vec4::{Color, Point, Vec4},
};

pub struct Dielectric {
    albedo: Color,
    // TODO: Support dynamic two way coupling (here the other material is presumed to be air)
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(albedo: Color, refractive_index: f32) -> Self {
        Self {
            albedo,
            refractive_index,
        }
    }

    /// Generates refracted ray
    pub fn refract(normal: Vec4, direction: Point, eta_i_by_eta_r: f32) -> Point {
        let cos_theta = direction.dot(normal).min(1.) * -1.;
        let r_out_perpendicular = (direction + normal * cos_theta) * eta_i_by_eta_r;
        let r_out_parallel =
            normal * -1. * (1.0 - r_out_perpendicular.length().powi(2)).abs().sqrt();
        r_out_perpendicular + r_out_parallel
    }
}

impl Material for Dielectric {
    fn generate_reflected_ray(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        // Normal adjustment done to keep calculations consistent for refraction for both out and in scenarios
        let (eta_i_by_eta_r, adjusted_normal) = if hit_record.normal.dot(ray.direction) < 0. {
            // ray coming from outside of object
            (1. / self.refractive_index, hit_record.normal)
        } else {
            (self.refractive_index, hit_record.normal * -1.)
        };

        // TODO: normalization can be avoided here
        let refracted_ray_direction =
            Self::refract(adjusted_normal, ray.direction, eta_i_by_eta_r).normalise();
        Some((
            self.albedo,
            Ray::new(
                hit_record.point_of_intersection,
                refracted_ray_direction,
            ),
        ))
    }
}
