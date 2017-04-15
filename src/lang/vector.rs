use core::fmt;
use core::hash::{Hasher};

use collection_traits::*;
use hash_map::DefaultHasher;
use vector;

use ::{Context, LHash, Ptr};

use super::object::Object;
use super::value::Value;
use super::scope::Scope;
use super::list::List;


pub const SHIFT: usize = 5;
pub const SIZE: usize = 1 << SHIFT;
pub const MASK: usize = SIZE - 1;


pub struct Vector {
    root: Ptr<Object<[Ptr<Value>; SIZE]>>,
    tail: Ptr<Object<[Ptr<Value>; SIZE]>>,
    size: Ptr<Object<usize>>,
    shift: usize,
}

impl LHash for Vector {

    #[inline(always)]
    fn hash(&self, state: &mut DefaultHasher) {
        ((&self) as *const _ as usize).hash(state);
    }
}

impl Vector {

    #[inline(always)]
    pub fn new(context: &Context) -> Self {
        Vector {
            root: Self::create_array(context),
            tail: Self::create_array(context),
            size: context.gc.new_object(context.UIntType, 0usize),
            shift: SHIFT,
        }
    }

    #[inline(always)]
    fn clone(&self, context: &Context) -> Self {
        Vector {
            root: self.root,
            tail: self.tail,
            size: context.gc.new_object(context.UIntType, *self.size.value()),
            shift: self.shift,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        let mut vector = Self::new(context);

        for value in args.iter() {
            vector.push_mut(context, value);
        }

        context.gc.new_object(context.VectorType, vector).as_value()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        *self.size.value()
    }

    #[inline]
    fn get_array(&self, index: usize) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        if index >= Self::tail_off(self.size()) {
            self.tail
        } else {
            let mut array = self.root;
            let mut level = self.shift;

            while level > 0 {
                array = array[((index >> level) & MASK)]
                    .downcast::<Object<[Ptr<Value>; SIZE]>>().unwrap();
                level = level - SHIFT;
            }

            array
        }
    }

    #[inline(always)]
    fn get_unchecked(&self, index: usize) -> Ptr<Value> {
        self.get_array(index)[index & MASK]
    }

    fn tail_off(size: usize) -> usize {
        if size < SIZE {
            0
        } else {
            ((size - 1) >> SHIFT) << SHIFT
        }
    }

    fn create_array(context: &Context) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        context.gc.new_null_typ_object([context.nil_value.as_value(); SIZE])
    }

    fn copy_array(
        a: Ptr<Object<[Ptr<Value>; SIZE]>>,
        mut b: Ptr<Object<[Ptr<Value>; SIZE]>>,
        length: usize
    ) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        {
            let a_array = a.value();
            let mut b_array = b.value_mut();

            for i in 0..length {
                b_array[i] = a_array[i];
            }
        }
        b
    }

    fn clone_array(context: &Context, array: Ptr<Object<[Ptr<Value>; SIZE]>>, length: usize) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        Self::copy_array(array, Self::create_array(context), length)
    }

    #[inline]
    fn new_path_set(
        context: &Context,
        array: Ptr<Object<[Ptr<Value>; SIZE]>>,
        size: usize, index: usize, value: Ptr<Value>, level: usize
    ) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        let mut new_array = Self::clone_array(context, array, ((size - 1) >> level) & MASK);

        if level == 0 {
            new_array[index & MASK] = value;
        } else {
            let sub_index = (index >> level) & MASK;
            let sub_array = array[sub_index].downcast::<Object<[Ptr<Value>; SIZE]>>().unwrap();
            new_array[sub_index] = Self::new_path_set(context, sub_array, size, index, value, level - SHIFT).as_value();
        }

        new_array
    }

    fn new_path(context: &Context, array: Ptr<Object<[Ptr<Value>; SIZE]>>, level: usize) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        if level == 0 {
            array
        } else {
            let mut new_array = Self::create_array(context);
            new_array[0] = Self::new_path(context, array, level - SHIFT).as_value();
            new_array
        }
    }

    fn push_tail(
        context: &Context,
        parent_array: Ptr<Object<[Ptr<Value>; SIZE]>>,
        tail_array: Ptr<Object<[Ptr<Value>; SIZE]>>,
        size: usize,
        level: usize
    ) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        let sub_index = ((size - 1) >> level) & MASK;
        let mut new_array = Self::clone_array(context, parent_array, sub_index);
        let array_to_insert;

        if level == SHIFT {
            array_to_insert = tail_array;
        } else {
            let child = parent_array[sub_index];

            if child.typ() == context.NilType {
                array_to_insert = Self::new_path(context, tail_array, level - SHIFT);
            } else {
                let child_array = child.downcast::<Object<[Ptr<Value>; SIZE]>>().unwrap();
                array_to_insert = Self::push_tail(context, child_array, tail_array, size, level - SHIFT);
            }
        }

        new_array[sub_index] = array_to_insert.as_value();

        return new_array;
    }

    #[inline]
    pub(crate) fn push_mut(&mut self, context: &Context, value: Ptr<Value>) {
        let root = self.root;
        let size = self.size();
        let shift = self.shift;

        if size - Self::tail_off(size) < SIZE {
            self.tail[size & MASK] = value;
        } else {
            let mut new_root;
            let tail_array = self.tail;
            let mut new_shift = shift;

            if (size >> SHIFT) > (1 << shift) {
                new_root = Self::create_array(context);
                new_root[0] = root.as_value();
                new_root[1] = Self::new_path(context, tail_array, shift).as_value();
                new_shift += SHIFT;
            } else {
                new_root = Self::push_tail(context, root, tail_array, size, shift);
            }

            let mut new_tail = Self::create_array(context);
            new_tail[0] = value;

            self.tail = new_tail;
            self.root = new_root;
            self.shift = new_shift;
        }

        *self.size.value_mut() = size + 1usize;
    }

    #[inline(always)]
    pub fn iter(&self) -> VectorIter {
        VectorIter {
            vector: self,
            index: 0,
            len: self.size(),
        }
    }
}


pub struct VectorIter<'a> {
    vector: &'a Vector,
    index: usize,
    len: usize,
}

impl<'a> Iterator for VectorIter<'a> {
    type Item = Ptr<Value>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let value = self.vector.get_unchecked(self.index);
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = self.len - self.index;
        (size, Some(size))
    }
}


impl fmt::Display for Vector {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        let mut it = self.iter();
        while let Some(value) = it.next() {
            let (size, _) = it.size_hint();

            if size > 0 {
                write!(f, "{:?} ", value)?;
            } else {
                write!(f, "{:?}", value)?;
            }
        }
        write!(f, "]")
    }
}

impl fmt::Debug for Vector {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Ptr<Object<Vector>> {

    #[inline(always)]
    pub fn clone(&self, context: &Context) -> Self {
        context.gc.new_object(context.VectorType, (&***self).clone(context))
    }

    #[inline(always)]
    pub fn size(&self) -> Ptr<Object<usize>> {
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

    #[inline]
    pub fn get(&self, index: Ptr<Object<usize>>, not_set_value: Ptr<Value>) -> Ptr<Value> {
        let index = *index.value();

        if index >= (&**self).size() {
            not_set_value
        } else {
            self.get_unchecked(index)
        }
    }

    #[inline]
    pub fn push(&self, context: &Context, value: Ptr<Value>) -> Self {
        let mut new_vector = self.clone(context);
        new_vector.tail = Vector::clone_array(context, self.tail, ((&**self).size() + 1) & MASK);
        new_vector.push_mut(context, value);
        new_vector
    }

    #[inline]
    fn set_unchecked(&mut self, context: &Context, size: usize, index: usize, value: Ptr<Value>) -> Self {
        let mut vector = self.clone(context);

        if index >= Vector::tail_off(size) {
            let mut tail = self.tail;
            let masked_index = index & MASK;

            tail = Vector::clone_array(context, tail, (size + 1) & MASK);
            tail[masked_index] = value;

            vector.tail = tail;
        } else {
            vector.root = Vector::new_path_set(context, self.root, size, index, value, self.shift);
        }

        vector
    }

    #[inline(always)]
    pub fn set(&mut self, context: &Context, index: Ptr<Object<usize>>, value: Ptr<Value>) -> Self {
        let size = (&**self).size();
        let index = *index.value();

        if index < size {
            self.set_unchecked(context, size, index, value)
        } else {
            *self
        }
    }

    #[inline]
    pub fn insert(&mut self, context: &Context, index: Ptr<Object<usize>>, value: Ptr<Value>) -> Self {
        let size = (&**self).size();
        let index = *index.value();

        if index < size {
            let mut vector: vector::Vector<Ptr<Value>> = self.iter().collect();
            vector.insert(index, value);

            let mut new_vector = Vector::new(context);
            for value in vector {
                new_vector.push_mut(context, value);
            }

            context.gc.new_object(context.VectorType, new_vector)
        } else {
            *self
        }
    }
}
