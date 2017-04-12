use collections::string::String;

use core::fmt;

use ::{Context, Ptr};

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
    size: Ptr<Object<u64>>,
    shift: usize,
}

impl Vector {

    #[inline]
    pub fn new(context: &Context) -> Self {
        Vector {
            root: create_array(context),
            tail: create_array(context),
            size: context.gc.new_object(context.UInt64Type, 0u64),
            shift: SHIFT,
        }
    }

    #[inline(always)]
    fn clone(&self, context: &Context) -> Self {
        Vector {
            root: self.root,
            tail: self.tail,
            size: context.gc.new_object(context.UInt64Type, *self.size.value()),
            shift: self.shift,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        context.gc.new_object(context.VectorType, Self::new(context)).as_value()
    }

    #[inline(always)]
    pub fn size(&self) -> usize {
        *self.size.value() as usize
    }

    #[inline]
    fn get_array(&self, index: usize) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        if index >= tail_off(self.size()) {
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

    #[inline]
    fn new_path_set(
        context: &Context,
        array: Ptr<Object<[Ptr<Value>; SIZE]>>,
        size: usize, index: usize, value: Ptr<Value>, level: usize
    ) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
        let mut new_array = clone_array(context, array, ((size - 1) >> level) & MASK);

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
            let mut new_array = create_array(context);
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
        let mut new_array = clone_array(context, parent_array, sub_index);
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
    fn push_mut(&mut self, context: &Context, value: Ptr<Value>) {
        let root = self.root;
        let size = self.size();
        let shift = self.shift;

        if size - tail_off(size) < SIZE {
            self.tail[size & MASK] = value;
        } else {
            let mut new_root;
            let tail_array = self.tail;
            let mut new_shift = shift;

            if (size >> SHIFT) > (1 << shift) {
                new_root = create_array(context);
                new_root[0] = root.as_value();
                new_root[1] = Self::new_path(context, tail_array, shift).as_value();
                new_shift += SHIFT;
            } else {
                new_root = Self::push_tail(context, root, tail_array, size, shift);
            }

            let mut new_tail = create_array(context);
            new_tail[0] = value;

            self.tail = new_tail;
            self.root = new_root;
            self.shift = new_shift;
        }

        *self.size.value_mut() = (size as u64) + 1;
    }
}

impl fmt::Display for Vector {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = self.size();

        write!(f, "[")?;
        for i in 0..size {
            if i != (size - 1) {
                write!(f, "{} ", self.get_unchecked(i))?;
            } else {
                write!(f, "{}", self.get_unchecked(i))?;
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

    pub fn get(&self, index: Ptr<Object<usize>>, not_set_value: Ptr<Value>) -> Ptr<Value> {
        let index = *index.value();

        if index >= (&**self).size() {
            not_set_value
        } else {
            self.get_unchecked(index)
        }
    }

    pub fn push(&self, context: &Context, value: Ptr<Value>) -> Self {
        let mut new_vector = self.clone(context);
        new_vector.tail = clone_array(context, self.tail, ((&**self).size() + 1) & MASK);
        new_vector.push_mut(context, value);
        new_vector
    }
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
    mut a: Ptr<Object<[Ptr<Value>; SIZE]>>,
    mut b: Ptr<Object<[Ptr<Value>; SIZE]>>,
    length: usize
) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
    {
        let mut a_array = a.value_mut();
        let mut b_array = b.value_mut();

        for i in 0..length {
            b_array[i] = a_array[i];
        }
    }
    b
}

fn clone_array(context: &Context, array: Ptr<Object<[Ptr<Value>; SIZE]>>, length: usize) -> Ptr<Object<[Ptr<Value>; SIZE]>> {
    copy_array(array, create_array(context), length)
}
