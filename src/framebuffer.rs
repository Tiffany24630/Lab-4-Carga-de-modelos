use crate::color::Color;
use sdl2::{pixels::PixelFormatEnum, render::Canvas, video::Window};

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
    depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height * 4],
            depth: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in self.pixels.chunks_exact_mut(4) {
            pixel[0] = color.r;
            pixel[1] = color.g;
            pixel[2] = color.b;
            pixel[3] = color.a;
        }

        self.depth.fill(f32::INFINITY);
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, depth: f32, color: Color) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        let pixel = (y as usize) * self.width + x as usize;
        if depth >= self.depth[pixel] {
            return;
        }

        self.depth[pixel] = depth;
        let index = pixel * 4;
        self.pixels[index] = color.r;
        self.pixels[index + 1] = color.g;
        self.pixels[index + 2] = color.b;
        self.pixels[index + 3] = color.a;
    }

    pub fn present(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        let creator = canvas.texture_creator();

        let mut texture = creator
            .create_texture_streaming(
                // El framebuffer guarda los bytes en orden R, G, B, A.
                // ABGR8888 usa ese orden en memoria en Windows little-endian.
                PixelFormatEnum::ABGR8888,
                self.width as u32,
                self.height as u32,
            )
            .map_err(|e| e.to_string())?;

        texture
            .update(None, &self.pixels, self.width * 4)
            .map_err(|e| e.to_string())?;

        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();

        Ok(())
    }
}
