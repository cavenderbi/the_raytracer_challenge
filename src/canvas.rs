use std::{fs::File, io::Write};

use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}
impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0.0, 0.0, 0.0); width * height],
        }
    }
    pub fn get_width(&self) -> usize { self.width }
    pub fn get_height(&self) -> usize { self.height }
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) -> () {
        self.pixels[y * self.width + x] = color;
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }
    fn ppm_header(&self, file: &mut File) -> () {
        file.write(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())
            .expect("Failed to write header to file.");
    }
    fn ppm_body(&self, file: &mut File) -> () {
        let mut i = 0;
        for pixel in &self.pixels {
            file.write(format!("{} ", pixel).as_bytes())
                .expect("Failed to write pixel to file.");
            if i >= 5 {
                file.write("\n".as_bytes())
                    .expect("Failed to write new-line character to file.");
                i = 0;
            } else {
                i += 1;
            }
        }
    }
    pub fn write_ppm(&self, filename: &'static str) -> () {
        let mut file = File::create(filename).expect("Failed to create file.");
        self.ppm_header(&mut file);
        self.ppm_body(&mut file);
    }
}
