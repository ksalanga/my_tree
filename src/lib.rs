use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    value: RefCell<T>,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Node<T> {
        Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        }
    }

    fn set_parent(&mut self, parent: Rc<Node<T>>) {
        *self.parent.borrow_mut() = Rc::downgrade(&parent);
    }

    fn add_child(&mut self, child: Rc<Node<T>>) {
        let mut children = self.children.borrow_mut();

        children.push(child);
    }
}

// TODO: Trees? Tree Traverser?
// TODO: How are we going to traverse nodes with parent and children

// pub struct Tree<T> {
//     root: Option<Rc<Node<T>>>,
//     current_node: Weak<Node<T>>,
// }

// impl<T> Tree<T> {
//     fn new() -> Tree<T> {
//         Tree {
//             root: None,
//             current_node: Weak::new(),
//         }
//     }
// }