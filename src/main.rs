mod pool;
mod utils;

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
    let (a, b) = (1, 2);
    let s: String = "hello".to_string();

    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 10, y: 20 };
    let Point { x: a, y: b } = p;
    println!("{}, {}", a, b);

    println!("PI={}", PI);

    const SIZE: usize = 3;
    let arr: [isize; SIZE] = [1, 2, 3];
    println!("arr: {:?}", arr);

    increment();
    println!("Counter: {}", COUNTER.load(Ordering::SeqCst));

    let _a: f32 = 0.1;
    let _c = 10f32;

    let _b = true;
    let _f = false;

    let _c = 'h';
    let _c = '%';
    let _c = 'A' as u32;
    println!("A unicode is {}", _c);

    let s = String::from("你好呀");
    println!("{}", s);

    let hello = String::from("السلام عليكم");
    println!("{}", hello);
    let hello = String::from("Dobrý den");
    println!("{}", hello);
    let hello = String::from("Hello");
    println!("{}", hello);
    let hello = String::from("שָׁלוֹם");
    println!("{}", hello);
    let hello = String::from("नमस्ते");
    println!("{}", hello);
    let hello = String::from("こんにちは");
    println!("{}", hello);
    let hello = String::from("안녕하세요");
    println!("{}", hello);
    let hello = String::from("你好");
    println!("{}", hello);
    let hello = String::from("Olá");
    println!("{}", hello);
    let hello = String::from("Здравствуйте");
    println!("{}", hello);
    let hello = String::from("Hola");
    println!("{}", hello);

    // 将""号进行转义
    let byte_escape = "I'm saying \"Hello\"";
    println!("{}", byte_escape);

    // 分两行打印
    let byte_escape = "I'm saying \n 你好";
    println!("{}", byte_escape);

    // Windows下的换行符
    let byte_escape = "I'm saying \r\n 你好";
    println!("{}", byte_escape);

    // 打印出 \ 本身
    let byte_escape = "I'm saying \\ Ok";
    println!("{}", byte_escape);

    // 强行在字符串后面加个0，与C语言的字符串一致。
    let byte_escape = "I'm saying hello.\0";
    println!("{}", byte_escape);

    let byte_escape = "hello \x77 \u{0065}";
    println!("{}", byte_escape);

    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    let longer_delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", longer_delimiter);

    let bs = b"wo zai";
    println!("bs is {:?}", bs);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = a[2];
    println!("{}", b);

    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3, 4];
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    println!("{:?}", v);

    let s1 = String::from("superman 1");
    let s2 = String::from("superman 2");
    let s3 = String::from("superman 3");
    let s4 = String::from("superman 4");

    let v = vec![s1, s2, s3, s4];
    // 这里下标访问越界了
    println!("{:?}", v[3]);

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("hello"), 1);
    scores.insert(String::from("hello1"), 12);
    let x = (12, String::from("hh"), 10u8);
    println!("{}, {}, {}", x.0, x.1, x.2);

    let v4 = IPADDRKIND::V4;
    let v6 = IPADDRKIND::V6;

    if 3 >= 5 {
        println!("3");
    }

    let mut t = Default::default();
    let num = 10;
    if num > 10 {
        t = 10;
    } else if num == 10 {
        t = 11;
    } else {
        t = 8;
    }

    println!("{}", t);

    let y = if t == 11 { t * 10 } else { t * 2 };
    println!("{}", y);

    let mut counter = 10;
    let res = loop {
        counter += 1;
        if counter == 100 {
            break counter * 10;
        }
    };

    println!("res {}", res);

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("stack pop: {}", top); // 输出 3、2、1
    }

    let mut n = 3;
    while n != 0 {
        n -= 1;
    }

    for n in 1..4 {
        println!("{n}")
    }
    println!();

    for n in 1..=4 {
        println!("{n}");
    }

    println!();
    for n in (1..4).rev() {
        println!("{n}");
    }

    println!();
    for n in (1..=4).rev() {
        println!("{n}");
    }
    println!();
    for ch in ('a'..='z').rev() {
        println!("{ch}");
    }
    fn ao(x: u32) -> u32 {
        x + 1
    }

    let ao1 = |x: u32| -> u32 { x + 1 };
    let ao2 = |x: u32| x + 1;
    let ao3 = |x: u32| x + 1;

    let a = 10u32; // 局部变量 
    let add_v2 = |x: u32| -> u32 { x + a }; // 定义一个闭包 
    let result2 = add_v2(20); // 调用闭包 
    println!("{}", result2);

    use utils::add;
    let c = add(3, 4);
    println!("c is {c}");

    pool::user::user();

    garden::gar();
    garden::vegetables::hello();
}

mod garden;

fn foo() -> u32 {
    10u32
}

#[cfg(test)]
mod tests {
    use crate::foo;
    #[test]
    fn it_works() {
        let res = foo();
        assert_eq!(res, 10u32);
    }

    #[test]
    fn it_not_works() {
        let res = foo();
        assert_ne!(res, 11u32);
    }
}

enum IPADDRKIND {
    V4,
    V6,
}
