#[allow(dead_code)]
fn fib_loop(n: u8) -> u8 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    let res = loop {
        (a, b) = fib_iter(a, b);
        i += 1;
        println!("next val is {}", b);

        if i >= n {
            break b;
        }
    };
    res
}

#[allow(dead_code)]
fn fib_while(n: u8) -> u8 {
    let (mut a, mut b, mut i) = (1u8, 1u8, 2u8);
    while i < n {
        (a, b) = fib_iter(a, b);
        i += 1;
        println!("next val is {}", b)
    }
    b
}

#[allow(dead_code)]
fn fib_for(n: u8) -> u8 {
    let (mut a, mut b) = (1u8, 1u8);

    for _i in 2..n {
        (a, b) = fib_iter(a, b);
        println!("next val is {}", b)
    }
    b
}

fn fib_iter(a: u8, b: u8) -> (u8, u8) {
    let c = a + b;
    (b, c)
}

#[cfg(test)]
mod test_fib {
    use super::*;

    #[test]
    fn test_fib_loop() {
        assert_eq!(fib_loop(5), 5);
        assert_eq!(fib_loop(6), 8);
        assert_eq!(fib_loop(7), 13);
        assert_ne!(fib_loop(7), 12);
    }

    #[test]
    fn test_fib_while() {
        assert_eq!(fib_while(5), 5);
        assert_eq!(fib_while(6), 8);
    }

    #[test]
    fn test_fib_for() {
        assert_eq!(fib_for(5), 5);
        assert_eq!(fib_for(6), 8);
    }
}

#[cfg(test)]
mod test_iter {
    #[test]
    fn test_iter() {
        let arr = [1, 2, 3];
        assert_eq!(arr[..], [1, 2, 3]);
        assert_eq!(arr[0..=1], [1, 2]);
        assert_eq!(arr[0..=2], [1, 2, 3]);
        assert_ne!(arr[0..=2], [1, 2, 3, 4]);
    }
}
