const PI: f64 = 3.1415629;
const MAX_USERS: usize = 1000;
const URL: &str = "www.baidu.com";
use std::sync::atomic::{AtomicU32, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);

fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn main() {
    println!("Hello, world!");
    print!("{:?}", "haha");
    println!();

    let x = 5;
    let mut y = 10;
    let (a,b) = (1,2);
    let s: String = "hello".to_string();

    struct Point{
        x: i32,
        y: i32,
    }
    let p = Point{x: 10, y:20};
    let Point{x: a,y: b} = p;
    println!("{}, {}", a, b);

    println!("PI={}", PI);

    const SIZE: usize = 3;
    let arr: [isize; SIZE] = [1,2,3];
    println!("arr: {:?}", arr);
    
    increment();
        println!("Counter: {}", COUNTER.load(Ordering::SeqCst));
}


