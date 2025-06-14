mod owner;

use crate::owner::{print_addr, print_vec_extend, test_up_ds};
use shared::utils;

fn main() {
    println!("Hello, world!");
    utils::echo_utils();
    shared::echo();

    print_addr();
    print_vec_extend();

    test_up_ds();
}
