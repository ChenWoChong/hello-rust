use std::{collections::HashMap, mem::size_of_val};

// 不带 move 时，闭包捕获的是对应自由变量的引用；
// 带 move 时，对应自由变量的所有权会被移动到闭包结构中
pub fn print_closure_size() {
    let c1 = || println!("hello world!");
    let c2 = |i: i32| println!("hello: {}", i);
    let name = String::from("chen");
    let name1 = name.clone();
    let mut table = HashMap::new();
    table.insert("hello", "world");
    let c3 = || println!("hell: {}", name);
    let c4 = move || println!("hello {}, {:?}", name1, table);
    let name2 = name.clone();
    let c5 = move || {
        let x = 1;
        let name3 = String::from("lindsey");
        println!("hello: {}, {:?}, {:?}", x, name2, name3);
    };

    println!(
        "closures:\n\tc1: {},\n\tc2: {},\n\tc3: {},\n\tc4: {},\n\tc5: {},\n\tmain: {}",
        size_of_val(&c1),
        size_of_val(&c2),
        size_of_val(&c3),
        size_of_val(&c4),
        size_of_val(&c5),
        size_of_val(&print_closure_size),
    );
}

pub fn call_fn_once() {
    let name = String::from("chen");
    let c = move |greeting: String| (greeting, name);

    let result = c("hello".to_string());
    println!("result: {:?}", result);

    // 闭包返回了 name， 所以消耗了 自由变量的所有权，只有调用一次
    // 闭包内部的数据一旦被转移，这个闭包就不完整了，也就无法再次使用
    // let result = c("world".to_string());
}

pub fn call_as_fn_once() {
    fn call_once(arg: String, c: impl FnOnce(String) -> (String, String)) -> (String, String) {
        c(arg)
    }

    let name = String::from("Tyr");

    // 这个闭包会 clone 内部的数据返回，所以它不是 FnOnce
    let c = move |greeting: String| (greeting, name.clone());

    // 所以 c1 可以被调用多次

    println!("c1 call once: {:?}", c("chen".into()));
    println!("c1 call twice: {:?}", c("wochong".into()));

    // 然而一旦它被当成 FnOnce 被调用，就无法被再次调用
    println!("result: {:?}", call_once("chen as once".into(), c));

    // 无法再次调用
    // let result = c("hi".to_string());

    // Fn 也可以被当成 FnOnce 调用，只要接口一致就可以
    println!("result: {:?}", call_once("not_closure".into(), not_closure));
    println!("result: {:?}", call_once("not_closure".into(), not_closure));
}

fn not_closure(arg: String) -> (String, String) {
    (arg, "Corey".into())
}

pub fn call_fn_mut() {
    let mut name = String::from("hello");
    let mut name1 = String::from("halo");

    let mut c = || {
        name.push_str("chen");
        println!("c: {}", name);
    };

    let mut c1 = || {
        name1.push_str("!");
        println!("c1: {}", name1);
    };

    c();
    c();
    c1();
    c1();

    fn call_mut(c: &mut impl FnMut()) {
        c();
    }

    fn call_once(c: impl FnOnce()) {
        c();
    }

    call_mut(&mut c);
    call_mut(&mut c);
    call_mut(&mut c1);
    call_mut(&mut c1);

    call_once(c);
    call_once(c1);
}
