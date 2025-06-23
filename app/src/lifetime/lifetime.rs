use std::collections::HashMap;

const NEW_WORLD: &str = "new_world";

pub fn strike<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s = suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

pub fn test_str_strike() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strike(&mut s1, ' ');
    println!("raw s: {},\n\tleft s1: {},\n\tsplit is: {}", s, s1, hello);
}

pub fn test_map() {
    let mut map = HashMap::new();
    map.insert("hello", "world");
    let key = "hello1";

    match map.get_mut(key) {
        Some(v) => do_something(v),
        None => {
            map.insert(key, "wochong");
        }
    }
}

fn do_something(v: &mut &str) {
    *v = NEW_WORLD;
}
