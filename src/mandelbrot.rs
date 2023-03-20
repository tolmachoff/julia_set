use num::Complex;

pub struct Mandelbrot {
    pub n: u32,
}

impl Mandelbrot {
    pub fn calc(&self, c: Complex<f64>) -> u32 {
        let mut z = Complex {re: 0.0, im: 0.0};
        for i in 0..self.n {
            z = z * z + c;
            if z.norm_sqr() >= 4.0 {
                return i;
            }
        }
        self.n
    }
}
