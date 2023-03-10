use crate::cameras::perspective_camera::PerspectiveCamera;
use crate::materials::dielectric::Dielectric;
use crate::materials::lambertian::Lambertian;
use crate::materials::metal::Metal;
use crate::objects::sphere::Sphere;
use crate::ray_tracer::{engine::Engine, interface::camera_base::Camera};
use crate::scene::Scene;
use crate::utils::vec4::{Color, Point, Vec4};
use std::rc::Rc;

mod cameras;
mod materials;
mod objects;
mod ray_tracer;
mod scene;
mod utils;

// * This is the rendered image (the canvas) dimensions
// * This as of now matches the virtual viewport AR for square pixels
//TODO: Having issues with 16/9 (non-integral values for AR)
const ASPECT_RATIO: f32 = 16.0 / 8.0;
const IMAGE_HEIGHT: u32 = 256;
const IMAGE_WIDTH: u32 = (ASPECT_RATIO * (IMAGE_HEIGHT as f32)) as u32;

const FOCAL_LENGTH: f32 = 1.0;

fn main() {
    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0, 1.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3, 1.0)));
    let mat_left = Rc::new(Dielectric::new(Color::new(1., 1., 1., 1.0), 1.5));
    let mat_left_inner = Rc::new(Dielectric::new(Color::new(1., 1., 1., 1.0), 1.5));
    let mat_right = Rc::new(Metal::new(
        Color::new(0.8, 0.6, 0.2, 1.),
        Some(0.9),
    ));

    let mut scene = Scene::new();
    scene.add(Box::new(Sphere::new(
        0.5,
        Point::new(0., 0., -1., 0.),
        mat_center,
    )));
    scene.add(Box::new(Sphere::new(
        0.5,
        Point::new(-1., 0., -1., 0.),
        mat_left,
    )));
    scene.add(Box::new(Sphere::new(
        -0.4,
        Point::new(-1., 0., -1., 0.),
        mat_left_inner,
    )));
    scene.add(Box::new(Sphere::new(
        0.5,
        Point::new(1., 0., -1., 0.),
        mat_right,
    )));
    scene.add(Box::new(Sphere::new(
        100.0,
        Point::new(0.0, -100.5, -1.0, 0.),
        mat_ground,
    )));

    let camera: Box<dyn Camera> = Box::new(PerspectiveCamera::new(
        ASPECT_RATIO,
        FOCAL_LENGTH,
        90.,
        Point::new(-2., 2., 1., 0.),
        Point::new(0., 0., -1., 0.),
        Vec4::new(0., 1., 0., 0.),
    ));
    let engine = Engine::new(
        camera,
        scene,
        IMAGE_HEIGHT,
        IMAGE_WIDTH,
        true,
        100,
    );
    let output: Vec<Vec<Color>> = engine.render();

    /* -------------------------------------------------------------------------- */
    /*                          WRITE IMAGE DATA TO FILE                          */
    /* -------------------------------------------------------------------------- */
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    // Following natural co-ordinate system (y up, x right)
    for row in (0..IMAGE_HEIGHT).rev() {
        for column in 0..IMAGE_WIDTH {
            // TODO: Move to PNG
            let pixel_color = output[row as usize][column as usize];
            println!("{}", pixel_color.format_color());
        }
    }
}
