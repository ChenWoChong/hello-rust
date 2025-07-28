use serde::Deserialize;
use std::borrow::Cow;
use url::Url;

#[allow(dead_code)]
pub fn print_url_kv() {
    let url = Url::parse("https://tyr.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = url.query_pairs();

    assert_eq!(pairs.count(), 3);

    let (mut k, v) = pairs.next().unwrap();
    // 因为 k, v 都是 Cow<str> 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 Borrowed
    // print_pairs((k, v)); // k 发生 move

    println!("before change key: {}, v: {}", k, v);
    // 当修改发生时，k 变成 Owned
    k.to_mut().push_str("_lala");

    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());
    // 在处理 extra=hello%20world 时，value 被处理成 "hello world"
    // 所以这里 value 是 Owned
    print_pairs(pairs.next().unwrap());
}

fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    println!("key: {}, value: {}", show_cow(pair.0), show_cow(pair.1));
}

fn show_cow(cow: Cow<str>) -> String {
    match cow {
        Cow::Borrowed(v) => format!("Borrowed( {} )", v),
        Cow::Owned(v) => format!("Owned( {} )", v),
    }
}

#[derive(Debug, Deserialize)]
struct User<'input> {
    #[serde(borrow)]
    name: Cow<'input, str>,
    #[allow(dead_code)]
    age: u8,
}

pub fn print_user_cow() {
    let input = r#"{ "name": "Tyr", "age": 18 }"#;
    let mut user: User = serde_json::from_str(input).unwrap();

    println!("---User Name---");
    match user.name {
        Cow::Borrowed(x) => println!("borrowed name: {}", x),
        Cow::Owned(ref x) => println!("owned name: {}", x),
    }
    user.name.to_mut().push_str("_change");
    match user.name {
        Cow::Borrowed(x) => println!("borrowed name: {}", x),
        Cow::Owned(x) => println!("owned name: {}", x),
    }
}
