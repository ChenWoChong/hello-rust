mod generic;
mod lifetime;
mod owner;

use crate::generic::{test_hash_map, test_map_key_name};
use crate::lifetime::{test_map, test_str_strike};
use crate::owner::*;
use generic::{
    example_mutex_hash_map, print_arr_vec, print_url_kv, print_user_cow, print_vtable, test_add,
    test_my_string, test_print_iter, test_print_slice, test_print_str, test_vec_box,
};
use shared::utils;

fn main() {
    println!("Hello, world!");
    utils::echo_utils();
    shared::echo();

    print_addr();
    print_vec_extend();

    test_up_ds();

    println!("------------------\nPrint data inner mut:");
    inner_mut();
    thread_move();
    thread_share();

    caller();

    test_str_strike();
    test_map();
    println!("\n-------------------vtable-------------------\n");
    print_vtable();

    // test_dyn_writer();
    println!("\n----------------add-complex-------------------\n");
    test_add();

    println!("\n----------------cow-print-------------------\n");
    print_url_kv();
    print_user_cow();

    println!("\n----------------example_mutex_hash_map-------------------\n");
    example_mutex_hash_map();

    println!("\n----------------test_my_string-------------------\n");
    test_my_string();

    println!("\n----------------print_arr_vec-------------------\n");
    print_arr_vec();
    test_print_slice();
    test_print_iter();
    test_vec_box();
    test_print_str();

    println!("\n----------------print_hash_map-------------------\n");
    test_hash_map();
    test_map_key_name();
}
