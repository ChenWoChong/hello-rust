pub fn square(val: i32) -> i32 {
    val * val
}

pub fn cube(val: i32) -> i32 {
    val * val * val
}

pub fn apply(val: i32, f: fn(val: i32) -> i32) -> i32 {
    f(val)
}
