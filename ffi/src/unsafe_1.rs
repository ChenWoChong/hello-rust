use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
struct Evil {
    data: Rc<RefCell<usize>>,
}

unsafe impl Send for Evil {}

#[cfg(test)]
mod tests {
    use crate::unsafe_1::Evil;
    use std::thread;

    #[test]
    fn unsafe_should_err() {
        let v = Evil::default();
        let v1 = v.clone();
        let v2 = v.clone();

        let t1 = thread::spawn(move || {
            let v3 = v.clone();
            let mut data = v3.data.borrow_mut();
            *data += 1;
            println!("v3:{:?}", data);
        });

        let t2 = thread::spawn(move || {
            let v4 = v1.clone();
            let mut data = v4.data.borrow_mut();
            *data += 1;
            println!("v4: {:?}", data);
        });

        t1.join().unwrap();
        t2.join().unwrap();

        let mut data = v2.data.borrow_mut();
        *data += 1;
        println!("v2: {:?}", data);
    }
}
