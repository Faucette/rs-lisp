use ::Ptr;
use ::Context;

use super::nil::Nil;
use super::value::Value;
use super::object::Object;


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
    pub fn new(context: &Context) -> Self {
        List {
            root: None,
            tail: None,
            size: context.gc.new_object(context.UInt64Type, 0usize),
        }
    }
}

impl Ptr<Object<List>> {

    #[inline(always)]
    pub fn size(&self) -> Ptr<Object<usize>> {
        self.size
    }

    #[inline(always)]
    pub fn is_empty(&self, context: &Context) -> Ptr<Object<bool>> {
        context.gc.new_object(context.BooleanType, **self.size == 0)
    }

    #[inline]
    pub fn push(&self, context: &Context, data: Ptr<Value>) -> Self {
        let mut new_list = context.gc.new_object(context.ListType, List::new(context));
        let new_node = Some(context.gc.new_null_typ_object(Node::new(self.root, data)));

        if self.tail.is_some() {
            new_list.tail = self.tail;
        } else {
            new_list.tail = new_node;
        }

        new_list.root = new_node;
        new_list.size = context.gc.new_object(context.UInt64Type, (**self.size) + 1);

        new_list
    }

    #[inline]
    pub(crate) fn push_back_mut(&mut self, context: &Context, data: Ptr<Value>) -> &mut Self {
        let new_node = Some(context.gc.new_null_typ_object(Node::new(None, data)));

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
    pub fn pop(&self, context: &Context) -> Self {
        let mut new_list = context.gc.new_object(context.ListType, List::new(context));
        let size = **self.size;

        if size > 1 {
            new_list.root = self.root.unwrap().next;
            new_list.tail = self.tail;
            new_list.size = context.gc.new_object(context.UInt64Type, size - 1);
        }

        new_list
    }

    #[inline]
    pub fn peek(&self, context: &Context) -> Ptr<Value> {
        match self.root {
            Some(ref root) => root.data,
            None => context.gc.new_object(context.NilType, Nil::new()).as_value(),
        }
    }

    #[inline(always)]
    pub fn first(&self, context: &Context) -> Ptr<Value> {
        self.peek(context)
    }
    #[inline]
    pub fn last(&self, context: &Context) -> Ptr<Value> {
        match self.tail {
            Some(ref tail) => tail.data,
            None => context.gc.new_object(context.NilType, Nil::new()).as_value(),
        }
    }
}
