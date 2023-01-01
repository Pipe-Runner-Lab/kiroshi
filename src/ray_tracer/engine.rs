use super::{
    camera_base::Camera,
    object_base::HitRecord,
    utils::{map_to_range, Ray},
};
use crate::scene::Scene;
use crate::utils::vec4::Color;
use rand::prelude::*;

const WHITE: Color = Color {
    e: [1., 1., 1., 1.],
};
const BLUE: Color = Color {
    e: [130. / 255., 170. / 255., 227. / 255., 1.],
};

const T_MIN: f32 = 0.;
const T_MAX: f32 = 100.;

pub struct Engine {
    camera: Box<dyn Camera>,
    scene: Scene,
    image_height: u32,
    image_width: u32,

    // anti-aliasing
    anti_aliasing: bool,
    anti_aliasing_sample_count: u32,
}

impl Engine {
    pub fn new(
        camera: Box<dyn Camera>,
        scene: Scene,
        image_height: u32,
        image_width: u32,
        anti_aliasing: bool,
        anti_aliasing_sample_count: u32,
    ) -> Self {
        Self {
            camera,
            scene,
            image_height,
            image_width,
            anti_aliasing,
            anti_aliasing_sample_count,
        }
    }

    pub fn ray_color(&self, ray: &Ray) -> Color {
        // keeps track of closest hit t value for the ray
        let mut closest_hit_record: Option<HitRecord> = None;

        // TODO: Optimise this loop and nesting
        for object in &self.scene.objects {
            match object.is_ray_hit(ray, T_MIN, T_MAX) {
                Some(hit_record) => match &closest_hit_record {
                    Some(temp_hit_record) => {
                        if hit_record.t < temp_hit_record.t {
                            closest_hit_record = Some(hit_record);
                        }
                    }
                    None => {
                        closest_hit_record = Some(hit_record);
                    }
                },
                None => (),
            };
        }

        if let Some(hit_record) = closest_hit_record {
            let r = map_to_range(hit_record.normal.x(), -1., 1., 0., 1.);
            let g = map_to_range(hit_record.normal.y(), -1., 1., 0., 1.);
            let b = map_to_range(hit_record.normal.z(), -1., 1., 0., 1.);
            return Color::new(r, g, b, 1.);
        }

        let y = ray.direction.normalise().y(); // -1 <= y <= 1
        let t = map_to_range(y, -1., 1., 0., 1.);

        WHITE * (1. - t) + BLUE * t
    }

    pub fn render(&self) -> Vec<Vec<Color>> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<Vec<Color>> = vec![];

        for row in 0..self.image_height {
            output.push(vec![]);
            for column in 0..self.image_width {
                let pixel_color = if self.anti_aliasing {
                    // TODO: Alpha calculation messed up during anti-aliasing
                    let mut temp_pixel_color = Color::new(0., 0., 0., 0.);
                    for _ in 0..self.anti_aliasing_sample_count {
                        // INFO: Minor improvement by adding -0.5
                        let u =
                            (column as f32 + (rng.gen::<f32>() - 0.5)) / self.image_width as f32;
                        let v = (row as f32 + (rng.gen::<f32>() - 0.5)) / self.image_height as f32;

                        let ray = self.camera.generate_ray(u, v);
                        temp_pixel_color += self.ray_color(&ray);
                    }
                    temp_pixel_color /= self.anti_aliasing_sample_count as f32;
                    temp_pixel_color
                } else {
                    let u = column as f32 / self.image_width as f32;
                    let v = row as f32 / self.image_height as f32;

                    let ray = self.camera.generate_ray(u, v);
                    self.ray_color(&ray)
                };

                output[row as usize].push(pixel_color);
            }
        }

        output
    }
}
