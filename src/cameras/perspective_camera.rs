use crate::ray_tracer::{interface::camera_base::Camera, utils::Ray};
use crate::utils::vec4::Point;

// The size of the virtual viewport if important since it is relative to the virtual world
pub struct PerspectiveCamera {
    focal_length: f32,
    aspect_ratio: f32,
    // * This is the virtual viewport (the image plane)
    viewport_height: u32,
    viewport_width: u32,

    origin: Point,
    lower_left_corner: Point,
    horizontal_offset_vec: Point, // offset vector from lower_left_corner to reach right edge
    vertical_offset_vec: Point,   // offset vector from lower_left_corner to reach top edge
}

impl PerspectiveCamera {
    pub fn new(aspect_ratio: f32, focal_length: f32, viewport_height: u32) -> Self {
        let viewport_width = (aspect_ratio * (viewport_height as f32)) as u32;

        Self {
            focal_length,
            aspect_ratio,
            viewport_height,
            viewport_width,

            // * Static origin as of now
            origin: Point::new(0.0, 0.0, 0.0, 0.0),
            horizontal_offset_vec: Point::new(viewport_width as f32, 0.0, 0.0, 0.0),
            vertical_offset_vec: Point::new(0.0, viewport_height as f32, 0.0, 0.0),

            // lower left corner of the virtual viewport
            lower_left_corner: Point::new(
                -(viewport_width as f32 / 2.0),
                -(viewport_height as f32 / 2.0),
                -focal_length,
                0.0,
            ),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, u: f32, v: f32) -> Ray {
        let direction = self.lower_left_corner +
            (self.horizontal_offset_vec * u) +
            (self.vertical_offset_vec * v) -
            self.origin;

        Ray::new(self.origin, direction)
    }
}
