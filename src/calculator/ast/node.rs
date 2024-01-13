use std::rc::Rc;
use std::cell::{RefCell, Ref};

#[derive(Debug, Clone)]
struct NodeInner<T> {
    value: Option<T>,
    children: Vec<Node<T>>,
    parent: Option<(Node<T>, usize)>,
}

#[derive(Debug)]
pub struct Node<T> {
    inner: Rc<RefCell<NodeInner<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: Option<T>) -> Self {
        Self {
            inner: Rc::new(RefCell::new(
                NodeInner {
                    value,
                    children: Vec::new(),
                    parent: None,
                }))
        }
    }

    pub fn value(&self) -> Ref<Option<T>> {
        let inner = self.inner.borrow();
        Ref::map(inner, |inner| &inner.value)
    }

    pub fn set_value(&mut self, value: T) {
        let mut inner = self.inner.borrow_mut();
        inner.value = Some(value);
    }

    pub fn children(&self) -> Ref<Vec<Node<T>>> {
        let inner = self.inner.borrow();
        Ref::map(inner, |inner| &inner.children)
    }

    pub fn len_children(&self) -> usize {
        let inner = self.inner.borrow();
        inner.children.len()
    }

    pub fn add_child(&mut self, mut child: Node<T>) {
        let index = self.len_children();
        child.set_parent(self.clone(), index);
        let mut inner = self.inner.borrow_mut();
        inner.children.push(child);
    }

    pub fn replace_child(&mut self, index: usize, mut child: Node<T>) {
        let mut inner = self.inner.borrow_mut();
        child.set_parent(self.clone(), index);
        inner.children[index] = child;
    }

    pub fn parent(&self) -> Option<(Node<T>, usize)> {
        let inner = self.inner.borrow();
        inner.parent.clone()
    }

    pub fn set_parent(&mut self, parent: Node<T>, index: usize) {
        let mut inner = self.inner.borrow_mut();
        inner.parent = Some((parent, index));
    }

    pub fn root(&self) -> Node<T> {
        let mut node = self.clone();
        loop {
            match node.parent() {
                Some((parent, _)) => node = parent,
                None => break,
            }
        }
        node
    }
}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> std::fmt::Display for Node<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let inner = self.inner.borrow();
        match inner.value.as_ref() {
            Some(value) => write!(f, "{}", value)?,
            None => write!(f, "None")?,
        }
        for (i, child) in inner.children.iter().enumerate() {
            write!(f, "\n{}: {}", i, child)?;
        }
        Ok(())
    }
}
