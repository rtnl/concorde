use crate::box2::Box2f;
use parking_lot::Mutex;
use std::sync::Arc;

pub type WindowNode = Node<Box2f>;

impl Default for WindowNode {
    fn default() -> Self {
        Self::new(Box2f::new_root())
    }
}

pub struct Node<T> {
    value: T,
    children: Vec<Arc<Mutex<Node<T>>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }

    pub fn add_child_value(&mut self, value: T) {
        self.children.push(Arc::new(Mutex::new(Self::new(value))));
    }

    pub fn get_child(&self, index: usize) -> Option<Arc<Mutex<Node<T>>>> {
        self.children.get(index).cloned()
    }
}
