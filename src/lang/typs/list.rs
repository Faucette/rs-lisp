use super::super::super::Ptr;
use super::super::LIST;
use super::super::{Value, Object};


struct Node {
    next: Option<Ptr<Object<Node>>>,
    data: Ptr<Value>,
}

impl Node {

    #[inline(always)]
    pub fn new(
        next: Option<Ptr<Object<Node>>>,
        data: Ptr<Value>
    ) -> Self {
        Node {
            next: next,
            data: data,
        }
    }
}


pub struct List {
    root: Option<Ptr<Object<Node>>>,
    tail: Option<Ptr<Object<Node>>>,
    size: usize,
}

impl List {

    #[inline(always)]
    pub fn new() -> Self {
        List {
            root: None,
            tail: None,
            size: 0usize,
        }
    }

    pub fn constructor(args: Ptr<Object<List>>) -> Ptr<Value> {
        Object::new(LIST, List::new())
    }

    #[inline]
    pub fn push(&self, data: Ptr<Value>) -> Self {
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
    pub fn peek(&self) -> Option<Ptr<Value>> {
        match self.root {
            Some(root) => Some(root.data),
            None => None,
        }
    }
}
