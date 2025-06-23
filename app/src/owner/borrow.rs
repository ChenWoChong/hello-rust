use std::cell::RefCell;
use std::mem;

pub fn print_addr() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().fold(0, |acc, x| acc + x)
}

#[allow(unused)]
fn test_wrapper() {
    let mut a = 0i32;
    println!("{a}");
    a = 2;
    println!("{a}");

    let print = |msg: String| println!("{}", msg); // 最后是 println! 语句，有分号
    let print = |msg: String| format!("{}", msg); // 最后是 println! 语句，有分号

    // 闭包返回 10（i32）
    let returns_i32 = || {
        let x = 5;
        x * 2 // 表达式（无分号），作为返回值
    };

    // 闭包返回 ()
    #[allow(unused_must_use)]
    let returns_unit = || {
        let x = 5;
        x * 2; // 语句（有分号），返回 ()
    };
    returns_unit();
}

pub fn print_vec_extend() {
    // capacity 是 1, len 是 0
    let mut v = vec![1];
    // capacity 是 8, len 是 0
    let v1: Vec<i32> = Vec::with_capacity(8);

    println!("----------");
    println!("----------");

    print_vec("v1", v1);

    // 我们先打印 heap 地址，然后看看添加内容是否会导致堆重分配
    println!("heap start: {:p}", &v[0] as *const i32);

    extend_vec(&mut v);

    // heap 地址改变了！这就是为什么可变引用和不可变引用不能共存的原因
    println!("new heap start: {:p}", &v[0] as *const i32);

    print_vec("v", v);
}

fn extend_vec(v: &mut Vec<i32>) {
    // Vec<T> 堆内存里 T 的个数是指数增长的，我们让它恰好 push 33 个元素
    // capacity 会变成 64
    (2..34).into_iter().for_each(|i| v.push(i));
}

fn print_vec<T>(name: &str, data: Vec<T>) {
    let p: [usize; 3] = unsafe { mem::transmute(data) };
    // 打印 Vec<T> 的堆地址，capacity，len
    println!(
        "{}: heap addr: 0x{:x},\tcap {},\tlen {}",
        name, p[0], p[1], p[2]
    );
}

pub fn inner_mut() {
    let data = RefCell::new(1);
    {
        let mut v = data.borrow_mut();
        *v += 1;
    }
    let cur_data = data.borrow();
    println!("data: {:?}", cur_data)
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn caller() {
    let string1 = String::from("long"); // 'string1 开始
    let result;
    {
        let string2 = String::from("shorty"); // 'string2 开始
        result = longest(string1.as_str(), string2.as_str()); // 'a 被具体化为 'string2 的生命周期(选择较短的生命周期)
        println!("The longest is {}", result); // Ok, result 在 'string2 结束前使用
    } // 'string2 结束！result (其生命周期 'a = 'string2) 在此失效

    // 错误！尝试在 'a 结束后使用 result。编译器报错！
    // println!("The longest is {}", result);
} // 'string1 结束
