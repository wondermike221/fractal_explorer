
use rayon::prelude::*;
use crate::corners::Corners;
use crate::complex::Complex;
use crate::utils::*;

#[derive(Debug, Copy, Clone)]
pub struct Mandelbrot {
    pub domain_range: Corners
}

impl Mandelbrot {
    pub fn generate_fractal(self, corners: Corners) -> Vec<u32> {
        let (x_min, y_min, x_max, y_max) = corners.unpack();
        let mut buffer: Vec<u32> = vec![0; crate::DEFAULT_WIDTH * crate::DEFAULT_HEIGHT];
        buffer.par_iter_mut().enumerate().for_each(|(i, b)| {
            let real = interpolate((i % crate::DEFAULT_WIDTH) as f64, 0., crate::DEFAULT_WIDTH as f64, x_min, x_max);
            let imag = interpolate((i / crate::DEFAULT_HEIGHT) as f64, 0., crate::DEFAULT_HEIGHT as f64, y_min, y_max);

            let c = Complex { real: real, imag: imag };
            let mut z = Complex { real: 0., imag: 0. };

            let mut n = 0;

            while z.magnitude() < 2. && n < crate::GENERATION_INFINITY {
                z = (z * z) + c;
                n += 1;
            }

            *b = crate::fill(n);
        });
        return buffer;
    }
}
