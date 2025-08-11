mod container;
mod copy;
mod cow;
mod generic;
mod hash_map;
mod iterator;
mod mutex_guard;
mod my_add;
mod my_linked_list;
mod my_string;
mod vtable;
mod try_gdb;
mod closure;

pub use my_add::test_add;

pub use vtable::*;

pub use cow::{print_url_kv, print_user_cow};

pub use mutex_guard::example_mutex_hash_map;

pub use container::{
    print_arr_vec, test_print_iter, test_print_slice, test_print_str, test_vec_box,
};

pub use my_string::test_my_string;

pub use hash_map::{test_btree_map, test_hash_map, test_map_key_name};

pub use closure::{print_closure_size,call_fn_once, call_as_fn_once, call_fn_mut};
