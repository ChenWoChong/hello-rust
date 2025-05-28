pub mod base;
use base::*;

pub trait Shaper {
    fn circumference(&self) -> f64;
    fn area(&self) -> f64;
}

impl Shaper for Rectangle {
    fn circumference(&self) -> f64 {
        (self.a + self.b) * 2.0
    }
    fn area(&self) -> f64 {
        self.a * self.b
    }
}

impl Shaper for Circle {
    fn circumference(&self) -> f64 {
        self.r * 2.0 * 3.14
    }
    fn area(&self) -> f64 {
        self.r * self.r * 3.14
    }
}

impl Shaper for Triangle {
    fn circumference(&self) -> f64 {
        self.a + self.b + self.c
    }
    fn area(&self) -> f64 {
        self.a * self.b * self.c
    }
}

pub fn echo_circumference<T: Shaper>(a: &T) {
    println!("ZhouChange is {}", a.circumference());
}

pub fn echo_area<T: Shaper>(a: &T) {
    println!("area is {}", a.area());
}
