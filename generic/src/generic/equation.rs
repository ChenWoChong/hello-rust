use std::fmt::Debug;
use std::marker::PhantomData;

#[derive(Default, Debug)]
pub struct Equation<IterMethod> {
    current: u32,
    _type: PhantomData<IterMethod>,
}

#[derive(Default, Debug)]
pub struct Liner;
#[derive(Default, Debug)]
pub struct Quadratic;

impl Iterator for Equation<Liner> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None;
        }
        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None;
        }
        Some(self.current * self.current)
    }
}

#[allow(dead_code)]
pub fn consumer_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> Iter,
    Iter: Iterator<Item = T>,
    T: Debug,
{
    for i in f(8) {
        println!("print iterator item: {:?}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liner() {
        let mut liner = Equation::<Liner>::default();
        assert_eq!(liner.next(), Some(1));
        assert_eq!(liner.next(), Some(2));
        assert_eq!(liner.next(), Some(3));
        assert_eq!(liner.next(), Some(4));
        assert_eq!(liner.next(), Some(5));
    }

    #[test]
    fn test_quadratic() {
        let mut quadratic = Equation::<Quadratic>::default();
        assert_eq!(quadratic.next(), Some(1));
        assert_eq!(quadratic.next(), Some(4));
        assert_eq!(quadratic.next(), Some(9));
        assert_eq!(quadratic.next(), Some(16));
        assert_eq!(quadratic.next(), Some(25));
    }

    #[test]
    fn test_consumer_iterator() {
        let f = |i| (0..=i).into_iter();
        consumer_iterator(f);
    }
}
