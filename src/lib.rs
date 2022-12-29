use std::cell::{Ref, RefCell};
use std::ptr;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Node<T>
where
    T: PartialEq,
{
    value: RefCell<T>,
    parent: RefCell<Weak<Node<T>>>,
    children: RefCell<Vec<Rc<Node<T>>>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
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

    pub fn value(&self) -> Ref<T> {
        self.value.borrow()
    }

    pub fn set_value(&self, value: T) {
        *self.value.borrow_mut() = value;
    }

    pub fn get_child(&self, value: T) -> Option<Weak<Node<T>>> {
        let children = self.children.borrow();

        let found_child = children.iter().find(|node| *node.value() == value);

        match found_child {
            Some(node) => Some(Rc::downgrade(&node)),
            None => None,
        }
    }

    pub fn get_children(&self) -> Ref<Vec<Rc<Node<T>>>> {
        self.children.borrow()
    }

    pub fn get_parent(&self) -> Weak<Node<T>> {
        self.parent.borrow().clone()
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

    #[test]
    fn set_value() {
        let a = Node::new(1);

        a.set_value(3);

        assert_eq!(*a.value(), 3);
    }

    #[test]
    fn get_child_none() {
        let a = Node::new(1);

        let b = Node::new(2);

        a.add_child(&a, b);

        assert!(a.get_child(3).is_none());
    }

    #[test]
    fn get_child() {
        let a = Node::new(1);

        let b = Node::new(2);

        let observer_b = Rc::clone(&b);

        a.add_child(&a, b);

        assert!(ptr::eq(
            a.get_child(2).unwrap().upgrade().unwrap().as_ref(),
            observer_b.as_ref()
        ));
    }

    #[test]
    fn get_children() {
        let a = Node::new(1);

        let b = Node::new(2);
        a.add_child(&a, b);

        assert_eq!(a.get_children().len(), 1);
    }

    #[test]
    fn get_parent() {
        let a = Node::new(1);

        let b = Node::new(2);

        let observer_b = Rc::clone(&b);

        a.add_child(&a, b);

        let binding = observer_b.get_parent().upgrade().unwrap();
        let b_parent = binding.as_ref();

        assert!(ptr::eq(b_parent, a.as_ref()));
    }
}
