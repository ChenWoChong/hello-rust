mod generic;
mod vtable;
mod my_add;
mod iterator;
mod copy;
mod my_linked_list;
mod cow;

pub use my_add::test_add;

pub use vtable::*;

pub use cow::{print_url_kv, print_user_cow};
