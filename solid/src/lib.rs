mod test1;
mod test2;
mod test3;
mod test4;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
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
    fn test_any_type_id() {
        use std::any::{Any, TypeId};

        fn is_string(s: &dyn Any) -> bool {
            TypeId::of::<String>() == s.type_id()
        }

        assert_eq!(is_string(&0), false);
        assert_eq!(is_string(&"wo chong"), false);
        assert_eq!(is_string(&"wo chong".to_string()), true);
    }
}
