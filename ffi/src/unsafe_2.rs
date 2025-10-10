use std::alloc::{GlobalAlloc, Layout, System};
use std::mem;
use std::str::Utf8Chunk;

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
    use crate::unsafe_2::{Bar, COUNTER, Foo, Nonsense};
    #[allow(unused_imports)]
    use std::mem::transmute;
    use std::thread;

    #[test]
    fn test_impl_unsafe_trait_and_unsafe_func() {
        let nonsense = Nonsense;
        nonsense.foo();
        unsafe {
            nonsense.bar();
        }
    }

    #[test]
    fn test_unwrap_raw_pointer() {
        let mut age = 18;
        let r1 = &age as *const i32;
        let r2 = &mut age as *mut i32;

        unsafe {
            println!("r1: {}, r2: {}", *r1, *r2);
        }
    }

    // #[test]
    // fn test_immutable_mutable_cant_coexist() {
    //     let mut age = 18;
    //     let r1 = &age;
    //     let r2 = &mut age;
    // 
    //     println!("r1: {}, r2: {}", *r1, *r2);
    // }

    // #[test]
    // fn test_change_raw_pointer() {
    //     let r1 = 0xdeadbeef as *mut u32;
    //     println!("so far so good!");
    //
    //     unsafe {
    //         *r1 += 1;
    //         println!("r1: {}", *r1);
    //     }
    // }

    // #[test]
    // fn test_call_libc_malloc() {
    //     let data = unsafe {
    //         let p = libc::malloc(8);
    //         let arr: &mut [u8; 8] = transmute(p);
    //         arr
    //     };
    // 
    //     data.copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8]);
    //     println!("data: {:?}", data);
    // 
    //     unsafe { libc::free(transmute(data)) };
    // }

    #[test]
    fn test_global_counter() {
        let t1 = thread::spawn(move || unsafe { COUNTER += 10 });
        let t2 = thread::spawn(move || unsafe {
            COUNTER *= 10;
        });

        t2.join().unwrap();
        t1.join().unwrap();

        // unsafe { println!("COUNTER: {}", COUNTER) }
    }
}

#[allow(dead_code)]
static mut COUNTER: usize = 1;

#[allow(dead_code)]
pub struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe {
            let data = System.alloc(layout);
            eprintln!("ALLOC: {:p}, size {}", &data, layout.size());
            data
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            System.dealloc(ptr, layout);
            eprintln!("FREE: {:p}, size {}", &ptr, layout.size());
        }
    }
}

// #[global_allocator]
// static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
pub fn from_utf8(v: &[u8]) -> Result<&str, Utf8Chunk<'_>> {
    Ok(unsafe { from_utf8_unchecked(v) })
}

#[allow(dead_code)]
pub unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
    unsafe { mem::transmute(v) }
}
