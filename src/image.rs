use anyhow::Result;

use crate::ray::vec3f;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<vec3f> for Color {
    fn from(value: vec3f) -> Self {
        Color::new(value.x, value.y, value.z)
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0., 0., 0.)
    }

    pub fn white() -> Color {
        Color::new(1., 1., 1.)
    }
}

#[derive(Debug)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Image {
        Image {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn save(&self, file_name: &str) -> Result<()> {
        let mut res = image::RgbImage::new(self.width as u32, self.height as u32);

        for (x, y, p) in res.enumerate_pixels_mut() {
            *p = image::Rgb([
                (self[y as usize][x as usize].r * 255.0) as u8,
                (self[y as usize][x as usize].g * 255.0) as u8,
                (self[y as usize][x as usize].b * 255.0) as u8,
            ]);
        }

        res.save(file_name)?;
        Ok(())
    }
}

impl std::ops::Index<usize> for Image {
    type Output = [Color];

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index * self.width..(index + 1) * self.width]
    }
}

impl std::ops::IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index * self.width..(index + 1) * self.width]
    }
}
