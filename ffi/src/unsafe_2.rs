use lazy_static::lazy_static;
use std::alloc::{GlobalAlloc, Layout, System};
use std::collections::HashMap;
use std::mem;
use std::str::Utf8Chunk;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;

lazy_static! {
    static ref STORE: Mutex<HashMap<&'static str, &'static [u8]>> = Mutex::new(HashMap::new());
}

#[allow(dead_code)]
static COUNTER: AtomicUsize = AtomicUsize::new(1);
#[allow(dead_code)]
static mut COUNTER_SIMPLE: usize = 1;

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
    use crate::unsafe_2::{Bar, COUNTER, COUNTER_SIMPLE, Foo, Nonsense, STORE, split};
    #[allow(unused_imports)]
    use std::mem::transmute;
    use std::sync::atomic::Ordering;
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
        let t1 = thread::spawn(move || unsafe { COUNTER_SIMPLE += 10 });
        let t2 = thread::spawn(move || unsafe {
            COUNTER_SIMPLE *= 10;
        });

        t2.join().unwrap();
        t1.join().unwrap();

        // unsafe { println!("COUNTER_SIMPLE: {}", COUNTER_SIMPLE) }
    }

    #[test]
    fn test_mutex_hashmap() {
        let t1 = thread::spawn(move || {
            let mut store = STORE.lock().unwrap();
            store.insert("hello", b"world");
        });

        let t2 = thread::spawn(move || {
            let mut store = STORE.lock().unwrap();
            store.insert("goodbye", b"world");
        });

        t1.join().unwrap();
        t2.join().unwrap();

        println!("store: {:#?}", STORE.lock().unwrap());
    }

    #[test]
    fn test_atomic_counter() {
        let t1 = thread::spawn(move || {
            COUNTER.fetch_add(10, Ordering::SeqCst);
        });
        let t2 = thread::spawn(move || {
            COUNTER
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |v| Some(v * 10))
                .unwrap();
        });

        t1.join().unwrap();
        t2.join().unwrap();

        println!("COUNTER: {}", COUNTER.load(Ordering::Relaxed));
    }

    #[test]
    fn test_unsafe_split() {
        let mut s = "chen! corey".to_string();
        let r = s.as_mut();
        if let Some((s1, s2)) = split(r, '!') {
            println!("s1: {}, s2: {}", s1, s2);
        }
    }
}

#[allow(dead_code)]
fn split(s: &str, sep: char) -> Option<(&str, &str)> {
    let pos = s.find(sep);
    pos.map(|pos| {
        let len = s.len();
        let sep_len = sep.len_utf8();
        unsafe { (s.get_unchecked(0..pos), s.get_unchecked(pos + sep_len..len)) }
    })
}

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
