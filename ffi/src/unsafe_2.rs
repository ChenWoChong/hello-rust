#[allow(dead_code)]
unsafe trait Foo {
    fn foo(&self);
}

#[allow(dead_code)]
trait Bar {
    unsafe fn bar(&self);
}

#[allow(dead_code)]
struct Nonsense;

unsafe impl Foo for Nonsense {
    fn foo(&self) {
        println!("foo!");
    }
}

impl Bar for Nonsense {
    unsafe fn bar(&self) {
        println!("bar!");
    }
}

#[cfg(test)]
mod tests {
    use crate::unsafe_2::{Bar, Foo, Nonsense};

    #[test]
    fn test_impl_unsafe_trait_and_unsafe_func() {
        let nonsense = Nonsense;
        nonsense.foo();
        unsafe {
            nonsense.bar();
        }
    }
}
