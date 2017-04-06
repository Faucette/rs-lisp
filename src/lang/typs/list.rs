use super::super::super::Ptr;
use super::super::Object;


struct Node<T> {
    next: Option<Ptr<Object<Node<T>>>>,
    data: Ptr<Object<T>>,
}

impl<T> Node<T> {

    #[inline(always)]
    pub fn new(
        next: Option<Ptr<Object<Node<T>>>>,
        data: Ptr<Object<T>>
    ) -> Self {
        Node {
            next: next,
            data: data,
        }
    }
}


pub struct List<T> {
    root: Option<Ptr<Object<Node<T>>>>,
    tail: Option<Ptr<Object<Node<T>>>>,
    size: usize,
}

impl<T> List<T> {

    #[inline(always)]
    pub fn new() -> Self {
        List {
            root: None,
            tail: None,
            size: 0usize,
        }
    }

    #[inline]
    pub fn push(&self, data: Ptr<Object<T>>) -> Self {
        let new_node = Some(Object::new_null_typ(Node::new(self.root, data)));
        let mut new_list = List::new();

        if self.tail.is_some() {
            new_list.tail = self.tail;
        } else {
            new_list.tail = new_node;
        }

        new_list.root = new_node;
        new_list.size = self.size + 1;

        new_list
    }

    #[inline]
    pub fn pop(&self) -> Self {
        let mut new_list = List::new();

        if self.size > 1 {
            new_list.root = self.root.unwrap().next;
            new_list.tail = self.tail;
            new_list.size = self.size - 1;
        }

        new_list
    }

    #[inline]
    pub fn peek(&self) -> Option<Ptr<Object<T>>> {
        match self.root {
            Some(root) => Some(root.data),
            None => None,
        }
    }
}
