use super::super::super::utils::Ptr;
use super::super::{LIST, UINT64};
use super::super::value::Value;
use super::super::object::Object;


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
    size: Ptr<Object<usize>>,
}

unsafe impl Send for List {}
unsafe impl Sync for List {}

impl List {

    #[inline(always)]
    pub fn new() -> Self {
        List {
            root: None,
            tail: None,
            size: Object::new(unsafe {UINT64}, 0usize),
        }
    }

    #[inline(always)]
    pub fn constructor(_args: Ptr<Object<List>>) -> Ptr<Value> {
        Object::new(unsafe {LIST}, List::new()).as_value()
    }

    #[inline(always)]
    pub fn size(&self) -> Ptr<Object<usize>> {
        self.size
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
        new_list.size = Object::new(unsafe {UINT64}, (**self.size) + 1);

        new_list
    }

    #[inline]
    pub fn pop(&self) -> Self {
        let mut new_list = List::new();
        let size = **self.size;

        if size > 1 {
            new_list.root = self.root.unwrap().next;
            new_list.tail = self.tail;
            new_list.size = Object::new(unsafe {UINT64}, size - 1);
        }

        new_list
    }

    #[inline]
    pub fn peek(&self) -> Option<Ptr<Value>> {
        match self.root {
            Some(ref root) => Some(root.data),
            None => None,
        }
    }
}
