use std::rc::Rc;

#[derive(Debug)]
#[allow(unused)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

#[allow(unused)]
impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, ds: Rc<Node>) {
        self.downstream = Some(ds)
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

pub fn test_up_ds() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(node4));

    node1.update_downstream(Rc::new(node3));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1:{:#?},\nnode2:{:#?}", node1, node2);
}

#[cfg(test)]
mod tests {}
