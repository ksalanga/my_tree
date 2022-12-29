use std::cell::RefCell;
use std::ptr;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    value: RefCell<T>,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Rc<Node<T>> {
        Rc::new(Node {
            value: RefCell::new(value),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        })
    }

    pub fn add_child(&self, self_ref: &Rc<Self>, child: Rc<Node<T>>) {
        assert!(ptr::eq(self, self_ref.as_ref()));

        *child.parent.borrow_mut() = Rc::downgrade(self_ref);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_child() {
        let a = Node::new(1);

        let b = Node::new(2);

        a.add_child(&a, b);

        assert_eq!(a.children.borrow().len(), 1);
    }

    #[test]
    fn add_child_has_parent() {
        let a = Node::new(1);

        let b = Node::new(2);

        a.add_child(&a, b);

        let a_children = a.children.borrow();
        let b = a_children.get(0).unwrap();

        let b_parent = b.parent.borrow();

        assert!(b_parent.ptr_eq(&Rc::downgrade(&a)));
    }

    #[test]
    #[should_panic]
    fn add_child_with_child_parent_pointer_to_wrong_parent() {
        let a = Node::new(1);

        let b = Node::new(2);
        let c = Node::new(2);

        a.add_child(&c, b);

        assert_eq!(a.children.borrow().len(), 1);
    }
}
