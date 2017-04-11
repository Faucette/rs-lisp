use collections::string::String;

use core::fmt;

use ::Ptr;
use ::Context;

use super::value::Value;
use super::object::Object;
use super::scope::Scope;


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

impl fmt::Display for Node {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ":list_node")
    }
}

impl fmt::Debug for Node {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}


pub struct List {
    root: Option<Ptr<Object<Node>>>,
    tail: Option<Ptr<Object<Node>>>,
    size: Ptr<Object<u64>>,
}

unsafe impl Send for List {}
unsafe impl Sync for List {}

impl List {

    #[inline(always)]
    pub fn new(context: &Context) -> Self {
        List {
            root: None,
            tail: None,
            size: context.gc.new_object(context.UInt64Type, 0u64),
        }
    }

    #[inline]
    pub fn constructor(_context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        args.as_value()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        *self.size.value() as usize
    }

    #[inline]
    fn find_node(&self, index: usize) -> Option<Ptr<Object<Node>>> {
        if index < self.size() {
            let mut node = self.root;
            let mut i = 0;

            while i < index {
                node.map(|n| node = (**n).next);
                i += 1;
            }
            node
        } else {
            None
        }
    }
}

impl Ptr<Object<List>> {

    #[inline(always)]
    pub fn size(&self) -> Ptr<Object<u64>> {
        self.size
    }

    #[inline(always)]
    pub fn is_empty(&self, context: &Context) -> Ptr<Object<bool>> {
        if self.size.value() == &0 {
            context.true_value
        } else {
            context.false_value
        }
    }

    #[inline(always)]
    fn nth(&self, context: &Context, index: Ptr<Object<u64>>) -> Ptr<Value> {
        match self.find_node(*index.value() as usize) {
            Some(ref node) => node.data,
            None => context.nil_value.as_value(),
        }
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
            None => context.nil_value.as_value(),
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
            None => context.nil_value.as_value(),
        }
    }
}

impl fmt::Display for List {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut node = self.root;

        write!(f, "(")?;
        while let Some(n) = node {
            let value = n.data;

            node = n.next;

            if node.is_none() {
                write!(f, "{:?}", value)?;
            } else {
                write!(f, "{:?} ", value)?;
            }
        }
        write!(f, ")")
    }
}

impl fmt::Debug for List {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
