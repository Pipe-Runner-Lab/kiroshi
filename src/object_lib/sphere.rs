use crate::ray_tracer::{object_base::ObjectBase, utils::Ray};
use crate::utils::vec4::Point;

pub struct Sphere {
    radius: f32,
    center: Point,
}

impl Sphere {
    pub fn new(radius: f32, center: Point) -> Self {
        Self { radius, center }
    }
}

impl ObjectBase for Sphere {
    fn is_ray_hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        
        discriminant > 0.
    }
}
