use super::{
    camera::PerspectiveCamera,
    utils::{map_to_range, Ray},
};
use crate::scene::Scene;
use crate::utils::vec4::Color;

const WHITE: Color = Color {
    e: [1., 1., 1., 1.],
};
const BLUE: Color = Color {
    e: [130. / 255., 170. / 255., 227. / 255., 1.],
};
const RED: Color = Color {
    e: [1., 0., 0., 1.],
};

const T_MIN: f32 = 0.;
const T_MAX: f32 = 100.;

pub struct Engine {
    camera: PerspectiveCamera,
    scene: Scene,
    image_height: u32,
    image_width: u32,
}

impl Engine {
    pub fn new(
        camera: PerspectiveCamera,
        scene: Scene,
        image_height: u32,
        image_width: u32,
    ) -> Self {
        Self {
            camera,
            scene,
            image_height,
            image_width,
        }
    }

    pub fn ray_color(&self, ray: &Ray) -> Color {
        for object in &self.scene.objects {
            match object.is_ray_hit(ray, T_MIN, T_MAX) {
                Some(hit_record) => {
                    let r = map_to_range(hit_record.normal.x(), -1., 1., 0., 1.);
                    let g = map_to_range(hit_record.normal.y(), -1., 1., 0., 1.);
                    let b = map_to_range(hit_record.normal.z(), -1., 1., 0., 1.);
                    return Color::new(r, g, b, 1.);
                }
                None => (),
            };
        }

        let y = ray.direction.normalise().y(); // -1 <= y <= 1
        let t = map_to_range(y, -1., 1., 0., 1.);

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
