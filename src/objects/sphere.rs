use crate::ray_tracer::{
    interface::{
        material_base::Material,
        object_base::{HitRecord, Object},
    },
    utils::Ray,
};
use crate::utils::vec4::Point;
use std::rc::Rc;

pub struct Sphere {
    radius: f32,
    center: Point,
    material: Rc<dyn Material>, // TODO: Why can't this not be done using just reference or Box
}

impl Sphere {
    pub fn new(radius: f32, center: Point, material: Rc<dyn Material>) -> Self {
        Self {
            radius,
            center,
            material,
        }
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
            let mut t = (-half_b - discriminant.sqrt()) / a; // check the first point
            if t < t_min || t_max < t {
                t = (-half_b + discriminant.sqrt()) / a; // check the 2nd point
                if t < t_min || t_max < t {
                    return None;
                }
            }

            let point_of_intersection = ray.at(t);
            let mut normal = (point_of_intersection - self.center).normalise();

            // if radius is negative, turn turn sphere inside out
            if self.radius < 0. {
                normal *= -1.;
            }

            Some(HitRecord {
                point_of_intersection,
                normal,
                t,
                material: Rc::clone(&self.material),
            })
        }
    }
}
