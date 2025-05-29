pub mod utils;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn echo() {
    println!("Hello from share lib");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn echo_unit() {
        assert_eq!(echo(), ())
    }
}
