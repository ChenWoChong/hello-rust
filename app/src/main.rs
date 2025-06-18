mod owner;

use crate::owner::*;
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
}
