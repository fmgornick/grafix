use crate::utils::vector::{Point3, Vec3};
use std::fs::File;

pub struct Image {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub image_height: u32,

    // camera
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub camera_center: Point3,

    // helper vars
    pub viewport_u: Vec3<f64>,
    pub viewport_v: Vec3<f64>,
    pub pixel_u: Vec3<f64>,
    pub pixel_v: Vec3<f64>,
    pub viewport_upper_left: Vec3<f64>,
    pub pixel_00: Vec3<f64>,
}

impl Image {
    pub fn new(
        image_width: u32,
        aspect_ratio: f64,
        viewport_height: f64,
        focal_length: f64,
    ) -> Image {
        let image_height: u32 = match (image_width as f64 / aspect_ratio) as u32 {
            0 => 1,
            x => x,
        };
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let camera_center: Point3 = Point3::zero();
        let viewport_u: Vec3<f64> = Vec3::new(viewport_width as f64, 0.0, 0.0);
        let viewport_v: Vec3<f64> = Vec3::new(0.0, -viewport_height as f64, 0.0);

        let pixel_u: Vec3<f64> = viewport_u / image_width as f64;
        let pixel_v: Vec3<f64> = viewport_v / image_height as f64;

        let viewport_upper_left: Vec3<f64> =
            Vec3::new(0.0, 0.0, -focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00: Vec3<f64> = viewport_upper_left + pixel_u / 2.0 + pixel_v / 2.0;

        Image {
            aspect_ratio,
            image_width,
            image_height,
            viewport_width,
            viewport_height,
            focal_length,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_u,
            pixel_v,
            viewport_upper_left,
            pixel_00,
        }
    }

    pub fn render(self) -> std::io::Result<()> {
        let f = File::create("image.ppm")?;
        for i in 1..self.image_width {
            for j in 1..self.image_height {
                let pixel_center =
                    self.pixel_00 + self.pixel_u * i as f64 + self.pixel_v * j as f64;
            }
        }
        Ok(())
    }
}
