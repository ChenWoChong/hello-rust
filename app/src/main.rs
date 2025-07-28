mod generic;
mod lifetime;
mod owner;

use crate::lifetime::{test_map, test_str_strike};
use crate::owner::*;
use generic::{print_url_kv, print_vtable, test_add};
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
}
