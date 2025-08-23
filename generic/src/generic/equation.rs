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
}
