use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Identifier<T> {
    inner: u64,
    tag: PhantomData<T>,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Product {
    id: Identifier<Self>,
}

#[derive(Default, Debug, Eq, PartialEq)]
pub struct User {
    id: Identifier<Self>,
}

#[cfg(test)]
mod tests {
    use crate::generic::identifier::{Product, User};

    #[test]
    fn user_product_id_should_not_eq() {
        let product = Product::default();
        let usr = User::default();
        // assert_eq!(product.id, usr.id);
        assert_eq!(product.id.inner, usr.id.inner);
    }
}
