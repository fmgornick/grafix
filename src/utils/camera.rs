use crate::utils::image::Image;
use crate::utils::vector::{Color, Point3, Ray, Vec3};

pub struct Camera<'a> {
    pub image: &'a mut Image<'a>,

    // camera
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub camera_center: Point3,

    // helper vars
    pub viewport_u: Vec3<f64>,
    pub viewport_v: Vec3<f64>,
    pub pixel_du: Vec3<f64>,
    pub pixel_dv: Vec3<f64>,
    pub viewport_upper_left: Vec3<f64>,
    pub pixel_00: Vec3<f64>,
}

impl Camera<'_> {
    pub fn new<'a>(image: &'a mut Image<'a>, viewport_height: f64, focal_length: f64) -> Camera {
        let viewport_width: f64 = viewport_height * (image.width as f64 / image.height as f64);

        let camera_center: Point3 = Point3::zero();
        let viewport_u: Vec3<f64> = Vec3::new(viewport_width as f64, 0.0, 0.0);
        let viewport_v: Vec3<f64> = Vec3::new(0.0, -viewport_height as f64, 0.0);

        let pixel_du: Vec3<f64> = viewport_u / image.width as f64;
        let pixel_dv: Vec3<f64> = viewport_v / image.height as f64;

        let viewport_upper_left: Vec3<f64> =
            camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00: Vec3<f64> = viewport_upper_left + pixel_du * 0.5 + pixel_dv * 0.5;

        Camera {
            image,
            viewport_width,
            viewport_height,
            focal_length,
            camera_center,
            viewport_u,
            viewport_v,
            pixel_du,
            pixel_dv,
            viewport_upper_left,
            pixel_00,
        }
    }

    pub fn render(self) -> Result<(), std::io::Error> {
        for j in 0..self.image.height {
            for i in 0..self.image.width {
                let pixel_center =
                    self.pixel_00 + self.pixel_du * i as f64 + self.pixel_dv * j as f64;
                let r = Ray::new(self.camera_center, pixel_center - self.camera_center);
                self.image.write_pixel(ray_color(r))?;
            }
        }
        Ok(())
    }
}

pub fn ray_color(ray: Ray) -> Color {
    if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray.clone()) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let y_pos = 0.5 * (ray.dir.unit().y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - y_pos) + Color::new(0.5, 0.7, 1.0) * y_pos
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> bool {
    let t = ray.dir.dot(center - ray.origin) / ray.dir.dot(ray.dir);
    (ray.at(t) - center).norm() <= radius
}
