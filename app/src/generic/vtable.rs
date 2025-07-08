use std::fmt::{Debug, Display};
use std::mem::transmute;

pub fn print_vtable() {
    let s1 = String::from("hellow world");
    let s2 = String::from("goodbye, world");

    let w1: &dyn Display = &s1;
    let w2: &dyn Debug = &s1;

    let w3: &dyn Display = &s2;
    let w4: &dyn Debug = &s2;

    let (addr1, vtable1): (usize, usize) = unsafe { transmute(w1) };
    let (addr2, vtable2): (usize, usize) = unsafe { transmute(w2) };
    let (addr3, vtable3): (usize, usize) = unsafe { transmute(w3) };
    let (addr4, vtable4): (usize, usize) = unsafe { transmute(w4) };

    println!(
        "s1: {:p}, s2: {:p}, print_vtable():{:p}",
        &s1, &s2, print_vtable as *const ()
    );
    println!("s1 Display:");
    println!("\ndata addr1:\t0x{:x},\nvtable1:\t0x{:x}\n", addr1, vtable1);
    println!("s1 Debug:");
    println!("\ndata addr2:\t0x{:x},\nvtable2:\t0x{:x}\n", addr2, vtable2);
    
    println!("s2 Display:");
    println!("\ndata addr3:\t0x{:x},\nvtable3:\t0x{:x}\n", addr3, vtable3);
    println!("s2 Debug:");
    println!("\ndata addr4:\t0x{:x},\nvtable4:\t0x{:x}\n", addr4, vtable4);
    
    assert_eq!(addr1, addr2);
    assert_eq!(addr3, addr4);
    
    assert_eq!(vtable1, vtable3);
    assert_eq!(vtable2, vtable4);
}
