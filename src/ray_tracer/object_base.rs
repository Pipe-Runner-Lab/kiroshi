use crate::ray_tracer::utils::Ray;

pub trait ObjectBase {
   fn is_ray_hit(&self, ray: &Ray) -> bool; 
}