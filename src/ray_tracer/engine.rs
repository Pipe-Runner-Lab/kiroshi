use super::camera::PerspectiveCamera;
use crate::utils::vec4::Color;

pub fn render(camera: PerspectiveCamera, image_height: u32, image_width: u32) -> Vec<Vec<Color>> {
    let mut output: Vec<Vec<Color>> = vec![];

    for row in 0..image_height {
        output.push(vec![]);
        for column in 0..image_width {
            let r = (column as f32) / ((image_width - 1) as f32);
            let g = (row as f32) / ((image_height - 1) as f32);
            let b = 0.25;

            let pixel_color = Color::new(r, g, b, 1.0);
            output[row as usize].push(pixel_color);
        }
    }

    output
}
