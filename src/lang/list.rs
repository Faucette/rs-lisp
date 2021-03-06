use core::fmt;
use core::hash::{Hash, Hasher};

use ::{Context, Ptr};

use super::value::Value;
use super::object::Object;
use super::scope::Scope;
use super::number::Number;


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

impl Hash for Node {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.next {
            Some(next) => Hash::hash(&*next, state),
            None => (),
        }
        Hash::hash(&self.data, state);
    }
}

impl PartialEq for Node {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(&self.next, &other.next) &&
        self.data.equals(other.data)
    }
}

impl Eq for Node {}

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
    size: Ptr<Object<Number>>,
}

unsafe impl Send for List {}
unsafe impl Sync for List {}

impl List {

    #[inline(always)]
    pub fn new(context: &Context) -> Self {
        List {
            root: None,
            tail: None,
            size: context.gc.new_object(context.NumberType, Number::from(0usize)),
        }
    }

    #[inline]
    pub fn constructor(_context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        args.as_value()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        let number: Number = *self.size.value();
        number.into()
    }
    /*
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
    */

    #[inline(always)]
    pub fn iter(&self) -> ListIter {
        ListIter {
            root: self.root,
            size: self.size(),
        }
    }
}

impl Ptr<Object<List>> {

    #[inline(always)]
    pub fn size(&self) -> Ptr<Object<Number>> {
        self.size
    }

    #[inline(always)]
    pub fn is_empty(&self, context: &Context) -> Ptr<Object<bool>> {
        if (&**self).size() == 0usize {
            context.true_value
        } else {
            context.false_value
        }
    }
    /*
    #[inline(always)]
    fn nth(&self, context: &Context, index: Ptr<Object<Number>>) -> Ptr<Value> {
        match self.find_node(*index.value()) {
            Some(ref node) => node.data,
            None => context.nil_value.as_value(),
        }
    }
    */

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
        new_list.size = context.gc.new_object(context.NumberType, Number::from((&**self).size() + 1usize));

        new_list
    }

    #[inline]
    pub fn push_back_mut(&mut self, context: &Context, data: Ptr<Value>) -> &mut Self {
        let new_node = Some(context.gc.new_null_typ_object(Node::new(None, data)));

        if self.tail.is_some() {
            self.tail.unwrap().next = new_node;
        } else {
            self.root = new_node;
        }

        self.tail = new_node;
        ***self.size = ((***self.size as usize) + 1usize) as f64;

        self
    }

    #[inline]
    pub fn pop(&self, context: &Context) -> Self {
        let mut new_list = context.gc.new_object(context.ListType, List::new(context));
        let size = (***self.size) as usize;

        if size > 1usize {
            new_list.root = self.root.unwrap().next;
            new_list.tail = self.tail;
            new_list.size = context.gc.new_object(context.NumberType, Number::from(size - 1));
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

pub struct ListIter {
    root: Option<Ptr<Object<Node>>>,
    size: usize,
}

impl Iterator for ListIter {
    type Item = Ptr<Value>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.root.map(|node| {
                self.size -= 1;
                self.root = node.next;
                node.data
            })
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl Hash for List {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.root {
            Some(root) => Hash::hash(&*root, state),
            None => (),
        }
        match self.tail {
            Some(tail) => Hash::hash(&*tail, state),
            None => (),
        }
        Hash::hash(&*self.size, state);
    }
}

impl PartialEq for List {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if self.size.value() == other.size.value() {
            let mut bit = other.iter();

            for a in self.iter() {
                match bit.next() {
                    Some(b) => if a.equals(b) {
                        return false;
                    },
                    None => return false,
                }
            }
            true
        } else {
            false
        }
    }
}

impl Eq for List {}

impl fmt::Display for List {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        let mut it = self.iter();
        while let Some(value) = it.next() {
            let (size, _) = it.size_hint();

            if size > 0 {
                write!(f, "{:?} ", value)?;
            } else {
                write!(f, "{:?}", value)?;
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
