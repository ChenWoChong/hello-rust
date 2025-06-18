use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug)]
#[allow(unused)]
struct Node {
    id: usize,
    downstream: Option<Rc<RefCell<Node>>>,
}

#[allow(unused)]
impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, ds: Rc<RefCell<Node>>) {
        self.downstream = Some(ds)
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

pub fn test_up_ds() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(RefCell::new(node4)));

    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1:{:#?},\nnode2:{:#?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3
        .borrow_mut()
        .update_downstream(Rc::new(RefCell::new(node5)));

    println!("cur node1: {:#?},cur node2: {:#?}", node1, node2);
}

pub fn thread_move() {
    let arr = vec![1];

    let handler = std::thread::spawn(move || {
        println!("thread arr: {:?}", arr);
    });

    handler.join().unwrap();
}

pub fn thread_share() {
    let arr = Arc::new(vec![1]);
    {
        let arr_copy = arr.clone();
        let handler = std::thread::spawn(move || println!("thread inner arr: {:?}", arr_copy));
        handler.join().unwrap()
    }

    println!("thread outer arr: {:?}", arr);
}

#[cfg(test)]
mod tests {}
