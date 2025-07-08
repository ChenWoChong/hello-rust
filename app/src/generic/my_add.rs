use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Self::new(real, imagine)
    }
}

impl Add for &Complex {
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

pub fn test_add() {
    let c1 = Complex::new(1.0, 1f64);
    let c2 = Complex::new(2f64, 3.0);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", c1 + c2);
    println!("{:?}", c1 + c2);
}
