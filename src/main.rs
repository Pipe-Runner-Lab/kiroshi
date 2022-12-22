const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for r in 0..IMAGE_HEIGHT {
        for c in 0..IMAGE_WIDTH {
            let r = (r as f64) / ((IMAGE_HEIGHT - 1) as f64);
            let g = (c as f64) / ((IMAGE_WIDTH - 1) as f64);
            let b = 0.25;

            let ir = (255.999 * r) as u32;
            let ig = (255.999 * g) as u32;
            let ib = (255.999 * b) as u32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
