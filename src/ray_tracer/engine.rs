use super::{camera::PerspectiveCamera, utils::Ray};
use crate::utils::vec4::Color;

const WHITE: Color = Color {
    e: [1., 1., 1., 1.],
};
const BLUE: Color = Color {
    e: [130. / 255., 170. / 255., 227. / 255., 1.],
};

pub struct Engine {
    camera: PerspectiveCamera,
    image_height: u32,
    image_width: u32,
}

impl Engine {
    pub fn new(camera: PerspectiveCamera, image_height: u32, image_width: u32) -> Self {
        Self {
            camera,
            image_height,
            image_width,
        }
    }

    pub fn ray_color(&self, ray: &Ray) -> Color {
        let y = ray.direction.normalise().y(); // -1 <= y <= 1
        let t = 0.5 * (y + 1.); // scaling => 0 <= t <= 1

        WHITE * (1. - t) + BLUE * t
    }

    pub fn render(&self) -> Vec<Vec<Color>> {
        let mut output: Vec<Vec<Color>> = vec![];

        for row in 0..self.image_height {
            output.push(vec![]);
            for column in 0..self.image_width {
                let u = column as f32 / self.image_width as f32;
                let v = row as f32 / self.image_height as f32;

                let ray = self.camera.generate_ray(u, v);

                let pixel_color = self.ray_color(&ray);

                output[row as usize].push(pixel_color);
            }
        }

        output
    }
}
