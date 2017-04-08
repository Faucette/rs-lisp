use super::super::super::utils::Ptr;
use super::super::value::Value;
use super::super::object::Object;
use super::typs::{LIST, UINT64};
use super::primitives::{Nil, Boolean_new};


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
    fn new() -> Self {
        List {
            root: None,
            tail: None,
            size: Object::new(unsafe {UINT64}, 0usize),
        }
    }

    #[inline(always)]
    pub fn new_empty() -> Ptr<Object<List>> {
        Object::new(unsafe {LIST}, List::new())
    }
}

impl Ptr<Object<List>> {

    #[inline(always)]
    pub fn size(&self) -> Ptr<Object<usize>> {
        self.size
    }

    #[inline(always)]
    pub fn is_empty(&self) -> Ptr<Object<bool>> {
        Boolean_new(**self.size == 0)
    }

    #[inline]
    pub fn push(&self, data: Ptr<Value>) -> Self {
        let mut new_list = List::new_empty();
        let new_node = Some(Object::new_null_typ(Node::new(self.root, data)));

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
    pub(crate) fn push_back_mut(&mut self, data: Ptr<Value>) -> &mut Self {
        let new_node = Some(Object::new_null_typ(Node::new(None, data)));

        if self.tail.is_some() {
            self.tail.unwrap().next = new_node;
        } else {
            self.root = new_node;
        }

        self.tail = new_node;
        **self.size += 1;

        self
    }

    #[inline]
    pub fn pop(&self) -> Self {
        let mut new_list = List::new_empty();
        let size = **self.size;

        if size > 1 {
            new_list.root = self.root.unwrap().next;
            new_list.tail = self.tail;
            new_list.size = Object::new(unsafe {UINT64}, size - 1);
        }

        new_list
    }

    #[inline]
    pub fn peek(&self) -> Ptr<Value> {
        match self.root {
            Some(ref root) => root.data,
            None => Nil::new().as_value(),
        }
    }
}
