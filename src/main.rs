mod func;
mod leetcode;
mod shape;
mod chatroom;
mod fib;

use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("Type name is {}", type_name::<T>());
}

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

    let a = String::from("Gate");
    print_type_of(&a);

    let mut strs = vec!["aa".to_string(), "bb".to_string(), "cc".to_string()];
    for n in strs.iter_mut() {
        n.insert(1, 'z');
    }

    for n in &strs {
        println!("{n}");
    }

    println!("{:?}", strs);

    println!("square {}", func::apply(6, func::square));
    println!("cube {}", func::apply(6, func::cube));

    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = {
        pi();
    };
    println!(
        "is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}",
        is_pi, is_unit1, is_unit2
    );

    chatroom::chat();
}

fn pi() -> f64 {
    3.1415926
}
fn not_pi() {
    3.1415926;
}
