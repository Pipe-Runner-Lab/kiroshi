use crate::ray_tracer::utils::Ray;
use crate::utils::vec4::{Point, Vec4};

pub struct HitRecord {
    pub normal: Vec4,
    pub point_of_intersection: Point,
    pub t: f32
}

pub trait ObjectBase {
    fn is_ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
