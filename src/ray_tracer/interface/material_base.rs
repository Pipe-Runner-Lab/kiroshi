use crate::utils::vec4::Color;
use super::{super::utils::Ray, object_base::HitRecord};

pub trait Material {
  fn generate_reflected_ray(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}