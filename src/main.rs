mod shape;

fn main() {
    let rec = shape::base::Rectangle { a: 1.0, b: 2.0 };
    shape::echo_circumference(&rec);
    shape::echo_area(&rec);

    let circle = shape::base::Circle { r: 3.0 };
    shape::echo_circumference(&circle);
    shape::echo_area(&circle);

    let triangle = shape::base::Triangle {
        a: 4.0,
        b: 2.0,
        c: 3.0,
    };
    shape::echo_circumference(&triangle);
    shape::echo_area(&triangle);
}
