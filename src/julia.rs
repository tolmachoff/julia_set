use num::Complex;
use num::complex::ComplexFloat;

pub struct Julia {
    pub c: Complex<f64>,
    pub n: u32,
}

impl Julia {
    pub fn calc(&self, z: Complex<f64>) -> u32 {
        let mut z = z;
        for i in 0..self.n {
            z = z * z + self.c;
            if z.abs() > 2.0 {
                return i;
            }
        }
        self.n
    }
}
