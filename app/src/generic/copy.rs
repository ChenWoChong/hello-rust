#[derive(Clone, Copy, Default, Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_print() {
        let p1 = Point::default();
        let p2 = p1;

        println!("{:?}", p1);
        println!("{:?}", p2);
    }
}
