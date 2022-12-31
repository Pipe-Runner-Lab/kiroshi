mod object_lib;
mod ray_tracer;
mod scene;
mod utils;
mod prelude {
    pub use crate::object_lib::sphere::Sphere;
    pub use crate::ray_tracer::camera::PerspectiveCamera;
    pub use crate::ray_tracer::engine::Engine;
    pub use crate::scene::Scene;
    pub use crate::utils::vec4::{Color, Point};
    pub use std::io::{stderr, Write};
}

use prelude::*;

// * This is the rendered image (the canvas) dimensions
// * This as of now matches the virtual viewport AR for square pixels
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = 256;
const IMAGE_WIDTH: u32 = (ASPECT_RATIO * (IMAGE_HEIGHT as f32)) as u32;

const FOCAL_LENGTH: f32 = 1.0;

fn main() {
    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(
        0.5,
        Point::new(0., 0., -1., 0.),
    )));
    scene.add(Box::new(Sphere::new(
        100.0,
        Point::new(0.0, -100.5, -1.0, 0.),
    )));

    let camera = PerspectiveCamera::new(ASPECT_RATIO, FOCAL_LENGTH, 2);
    let engine = Engine::new(camera, scene, IMAGE_HEIGHT, IMAGE_WIDTH);
    let output: Vec<Vec<Color>> = engine.render();

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
