use std::rc::Rc;
use crate::ray_tracer::{
    interface::{object_base::{HitRecord, Object}, material_base::Material},
    utils::Ray,
};
use crate::utils::vec4::Point;

pub struct Sphere {
    radius: f32,
    center: Point,
    material: Rc<dyn Material> // TODO: Why can't this not be done using just reference or Box
}

impl Sphere {
    pub fn new(radius: f32, center: Point, material: Rc<dyn Material>) -> Self {
        Self { radius, center, material }
    }
}

impl Object for Sphere {
    fn is_ray_hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let half_b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            None
        } else {
            let t = (-half_b - discriminant.sqrt()) / a;
            if t < t_min || t_max < t{
                return None;
            }

            let point_of_intersection = ray.at(t);
            let normal = (point_of_intersection - self.center).normalise();
            Some(HitRecord {
                point_of_intersection,
                normal,
                t,
                material: Rc::clone(&self.material)
            })
        }
    }
}
