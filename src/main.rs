mod ray_tracer;
mod utils;
mod prelude {
    pub use crate::utils::vec4::{Color, Point};
    pub use crate::ray_tracer::camera::PerspectiveCamera;
    pub use crate::ray_tracer::engine::render;
}

use prelude::*;
use std::io::{stderr, Write};

// * This is the rendered image (the canvas) dimensions
// * This as of now matches the virtual viewport AR for square pixels
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = 256;
const IMAGE_WIDTH: u32 = (ASPECT_RATIO * (IMAGE_HEIGHT as f32)) as u32;

const FOCAL_LENGTH: f32 = 1.0;

fn main() {
    let camera = PerspectiveCamera::new(ASPECT_RATIO, FOCAL_LENGTH, 2);
    let output = render(camera, IMAGE_HEIGHT, IMAGE_WIDTH);

    /* -------------------------------------------------------------------------- */
    /*                          WRITE IMAGE DATA TO FILE                          */
    /* -------------------------------------------------------------------------- */
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    // Following natural co-ordinate system (y up, x right)
    for row in (0..IMAGE_HEIGHT).rev() {
        // * Notice the use of stderr here, since
        // * stdout is used for the image data.
        eprint!("\rScanlines remaining: {:3}", row);
        stderr().flush().unwrap();

        for column in 0..IMAGE_WIDTH {
            let pixel_color = output[row as usize][column as usize];
            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\nDone");
}
