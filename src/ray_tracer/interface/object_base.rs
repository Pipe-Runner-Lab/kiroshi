use std::rc::Rc;
use crate::ray_tracer::utils::Ray;
use crate::utils::vec4::{Point, Vec4};
use super::material_base::Material;

pub struct HitRecord {
    pub normal: Vec4,
    pub point_of_intersection: Point,
    pub t: f32,
    pub material: Rc<dyn Material> 
}

pub trait Object {
    fn is_ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
