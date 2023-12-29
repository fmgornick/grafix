use grafix::utils::camera::Camera;
use grafix::utils::image::Image;
use std::fs::File;

static FILENAME: &str = "image.ppm";
static ASPECT_RATIO: f64 = 16.0 / 9.0;
static IMAGE_WIDTH: u32 = 400;
static VIEWPORT_HEIGHT: f64 = 2.0;
static FOCAL_LENGTH: f64 = 1.0;

fn main() -> std::io::Result<()> {
    let mut f = File::create(FILENAME)?;
    let mut image = Image::new(IMAGE_WIDTH, ASPECT_RATIO, FILENAME, &mut f);
    let camera = Camera::new(&mut image, VIEWPORT_HEIGHT, FOCAL_LENGTH);
    camera.render()?;
    Ok(())
}
