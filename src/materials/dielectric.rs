use crate::{
    ray_tracer::{
        interface::{material_base::Material, object_base::HitRecord},
        utils::Ray,
    },
    utils::vec4::{Color, Point, Vec4},
};
use rand::prelude::*;

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
    pub fn refract(normal: Vec4, direction: Point, relative_refractive_index: f32) -> Point {
        let cos_theta = direction.dot(normal).min(1.) * -1.;
        let r_out_perpendicular = (direction + normal * cos_theta) * relative_refractive_index;
        let r_out_parallel =
            normal * -1. * (1.0 - r_out_perpendicular.length().powi(2)).abs().sqrt();
        r_out_perpendicular + r_out_parallel
    }

    fn reflectance(cosine: f32, relative_refractive_index: f32) -> f32 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - relative_refractive_index) / (1.0 + relative_refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn generate_reflected_ray(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        // Normal adjustment done to keep calculations consistent for refraction for both out and in scenarios
        let (relative_refractive_index, adjusted_normal) = if hit_record.normal.dot(ray.direction) < 0. {
            // ray coming from outside of object
            (1. / self.refractive_index, hit_record.normal)
        } else {
            (self.refractive_index, hit_record.normal * -1.)
        };

        // TIR
        let sin_theta = (1. - adjusted_normal.dot(ray.direction).powi(2)).sqrt();
        if relative_refractive_index * sin_theta > 1. {
            let new_dir = ray.direction - hit_record.normal * ray.direction.dot(hit_record.normal) * 2.;

            // TODO: TIR Albedo?
            return Some((
                self.albedo,
                Ray::new(
                    hit_record.point_of_intersection,
                    new_dir,
                ),
            ));
        }

        // schlick approximation (reflectance)
        let mut rng = rand::thread_rng();
        let cos_theta = adjusted_normal.dot(ray.direction) * -1.;
        if rng.gen::<f32>() < Self::reflectance(cos_theta, relative_refractive_index) {
            let new_dir = ray.direction - hit_record.normal * ray.direction.dot(hit_record.normal) * 2.;

            // TODO: Reflectance Albedo?
            return Some((
                self.albedo,
                Ray::new(
                    hit_record.point_of_intersection,
                    new_dir,
                ),
            ));
        }


        // TODO: normalization can be avoided here
        let refracted_ray_direction =
            Self::refract(adjusted_normal, ray.direction, relative_refractive_index).normalise();
        Some((
            self.albedo,
            Ray::new(
                hit_record.point_of_intersection,
                refracted_ray_direction,
            ),
        ))
    }
}
