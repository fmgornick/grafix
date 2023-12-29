use crate::utils::vector::Color;
use std::fs::File;
use std::io::Write;
use std::process::exit;

#[allow(dead_code)]
enum ImageType {
    Invalid,
    JPEG,
    PNG,
    PPM,
}

pub struct Image<'a> {
    pub width: u32,
    pub height: u32,
    // pub aspect_ratio: f64,
    image_type: ImageType,
    file: &'a mut File,
}

impl Image<'_> {
    pub fn new<'a>(
        width: u32,
        aspect_ratio: f64,
        filename: &'a str,
        file: &'a mut File,
    ) -> Image<'a> {
        let height = match (width as f64 / aspect_ratio).round() as u32 {
            0 => 1,
            x => x,
        };
        let image_type = match filename.split('.').last() {
            Some("jpeg") => ImageType::JPEG,
            Some("jpg") => ImageType::JPEG,
            Some("png") => ImageType::PNG,
            Some("ppm") => ImageType::PPM,
            _ => ImageType::Invalid,
        };
        let mut image = Image {
            width,
            height,
            // aspect_ratio,
            image_type,
            file,
        };

        if let Err(_) = image.init_file() {
            exit(1);
        };
        image
    }

    pub fn init_file(&mut self) -> Result<(), std::io::Error> {
        match self.image_type {
            ImageType::PPM => {
                writeln!(self.file, "P3")?;
                writeln!(self.file, "{} {}", self.width, self.height)?;
                writeln!(self.file, "255")?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn write_pixel(&mut self, color: Color) -> Result<(), std::io::Error> {
        match self.image_type {
            ImageType::PPM => {
                writeln!(
                    self.file,
                    "{} {} {}",
                    (color.r() * 255.0).round() as u8,
                    (color.g() * 255.0).round() as u8,
                    (color.b() * 255.0).round() as u8,
                )?;
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
