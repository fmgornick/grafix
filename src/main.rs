use grafix::utils::color::Color;
use grafix::utils::image::Image;

static ASPECT_RATIO: f64 = 16.0 / 9.0;
static IMAGE_WIDTH: u32 = 256;
static VIEWPORT_HEIGHT: f64 = 6.0;
static FOCAL_LENGTH: f64 = 1.0;

fn main() -> std::io::Result<()> {
    let image: Image = Image::new(IMAGE_WIDTH, ASPECT_RATIO, VIEWPORT_HEIGHT, FOCAL_LENGTH);
    image.render()?;
    Ok(())
}
