use crate::ray_tracer::{interface::camera_base::Camera, utils::Ray};
use crate::utils::vec4::{Point, Vec4};

// The size of the virtual viewport if important since it is relative to the virtual world
pub struct PerspectiveCamera {
    focal_length: f32,
    aspect_ratio: f32,
    // * This is the virtual viewport (the image plane)
    viewport_height: f32,
    viewport_width: f32,

    origin: Point,
    lower_left_corner: Point,
    horizontal_offset_vec: Point, // offset vector from lower_left_corner to reach right edge
    vertical_offset_vec: Point,   // offset vector from lower_left_corner to reach top edge
}

impl PerspectiveCamera {
    // vfov in degrees
    pub fn new(
        aspect_ratio: f32,
        focal_length: f32,
        vfov: f32,
        look_from: Point,
        look_at: Point,
        view_up: Vec4, // used to change roll of camera
    ) -> Self {
        // local camera direction vectors
        let local_z = (look_from - look_at).normalise();
        let local_x = view_up.cross(local_z).normalise();
        let local_y = local_z.cross(local_x);

        let theta = std::f32::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        // calculated on the plane set at origin, not the view port
        let horizontal_offset_vec = local_x * viewport_width;
        let vertical_offset_vec = local_y * viewport_height;

        Self {
            focal_length,
            aspect_ratio,
            viewport_height,
            viewport_width,

            // * Static origin as of now
            origin: look_from,
            horizontal_offset_vec,
            vertical_offset_vec,

            // lower left corner of the virtual viewport
            lower_left_corner: look_from -
                local_z * focal_length -
                (horizontal_offset_vec / 2.) -
                (vertical_offset_vec / 2.),
        }
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, u: f32, v: f32) -> Ray {
        let direction = (self.lower_left_corner +
            (self.horizontal_offset_vec * u) +
            (self.vertical_offset_vec * v) -
            self.origin)
            .normalise();

        Ray::new(self.origin, direction)
    }
}
