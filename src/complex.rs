use {std::ops::Add, std::ops::Mul};


#[derive(Debug, Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub imag: f64
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.imag * other.real + self.real * other.imag
        }
    }
}

impl Complex {
    pub fn magnitude(self) -> f64 {
        return (self.real.powf(2.) + self.imag.powf(2.)).sqrt();
    }
}
