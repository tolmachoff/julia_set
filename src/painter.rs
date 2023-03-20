#[path ="mandelbrot.rs"]
mod mandelbrot;
use mandelbrot::Mandelbrot;

use num::Complex;

pub struct Painter {
    width: u32,
    height: u32,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    fractal: Mandelbrot
}

impl Painter {
    pub fn new(w: u32, h: u32) -> Self {
        Painter { 
            width: w, 
            height: h,
            x0: -2.0,
            x1: 2.0,
            y0: -2.0,
            y1: 2.0, 
            fractal: Mandelbrot { n: 128 } 
        }
    }

    fn scale_x(&self, i: u32) -> f64 {
        (i as f64) / (self.width as f64) * (self.x1 - self.x0) + self.x0
    }

    fn scale_y(&self, i: u32) -> f64 {
        (i as f64) / (self.height as f64) * (self.y0 - self.y1) + self.y1
    }

    fn apply_palette(&self, n: u32) -> (u8, u8, u8) {
        if n == self.fractal.n {
            (0, 0, 0)
        }
        else {
            let b = ((n as f64) / ((self.fractal.n - 1) as f64) * 255.0) as u8;
            (b, b, 128 + b / 2)
        }
    }

    pub fn get_pixel(&self, i: u32, j: u32) -> (u8, u8, u8) {
        let z = Complex {re: self.scale_x(i), im: self.scale_y(j)};
        let n = self.fractal.calc(z);
        self.apply_palette(n)
    }

    pub fn resize(&mut self, i: u32, j: u32) {
        let x = self.scale_x(i);
        let y = self.scale_y(j);
        let x_dist = self.x1 - self.x0;
        let y_dist = self.y1 - self.y0;
        let scale = 0.75;
        self.x0 = x - 0.5 * scale * x_dist;
        self.x1 = x + 0.5 * scale * x_dist;
        self.y0 = y - 0.5 * scale * y_dist;
        self.y1 = y + 0.5 * scale * y_dist;
    }

    /*pub fn get_image(&self) -> RgbImage {
        let mut buffer = RgbImage::new(self.width, self.height);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            *pixel = self.get_pixel(i, j);
        }
        buffer
    }*/
}
