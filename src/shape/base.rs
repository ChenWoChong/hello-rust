pub struct Rectangle {
    pub a: f64,
    pub b: f64,
}

pub struct Circle {
    pub r: f64,
}

pub struct Triangle {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

pub enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
    Triangle(Triangle),
}
