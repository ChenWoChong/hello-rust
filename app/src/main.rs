mod owner;
mod lifetime;

use crate::owner::*;
use shared::utils;
use crate::lifetime::{test_map, test_str_strike};

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
}
