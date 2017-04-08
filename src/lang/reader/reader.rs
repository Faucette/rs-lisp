use collection_traits::*;
use vector::Vector;

use ::Ptr;
use ::Context;
use ::lang::{Object, Function, List};

use super::list_reader::list_reader;
use super::symbol_reader::symbol_reader;
use super::whitespace_reader::whitespace_reader;
use super::number_reader::number_reader;


pub struct Reader {
    readers: Vector<Ptr<Function>>,
    input: Vector<char>,
    index: usize,
    row: u64,
    col: u64,
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl Reader {

    #[inline]
    pub fn new(input: Vector<char>) -> Self {
        let mut readers = Vector::new();

        readers.push(Function::new(list_reader));
        readers.push(Function::new(whitespace_reader));
        readers.push(Function::new(number_reader));
        readers.push(Function::new(symbol_reader));

        Reader {
            readers: readers,
            input: input,
            index: 0usize,
            row: 1u64,
            col: 1u64,
        }
    }

    #[inline]
    pub fn read(&mut self) -> char {
        let ch = self.peek(0);

        if ch == '\n' {
            self.row += 1;
            self.col = 1;
        } else if self.index != 0 {
            self.col += 1;
        }

        self.index += 1;

        ch
    }

    #[inline(always)]
    pub fn done(&self) -> bool {
        self.index >= self.input.len()
    }

    #[inline(always)]
    pub fn can_read(&self, offset: usize) -> bool {
        (self.index + offset) < self.input.len()
    }

    #[inline(always)]
    pub fn peek(&self, offset: usize) -> char {
        unsafe {
            *self.input.get_unchecked(self.index + offset)
        }
    }
}

impl Ptr<Object<Reader>> {
    #[inline]
    pub fn next(&mut self, context: &Context) -> Ptr<Object<List>> {
        if self.done() {
            let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
            ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, false).as_value());
            ret_list
        } else {
            for reader in self.readers.iter() {
                let mut args = context.gc.new_object(context.ListType, List::new(context));
                args.push_back_mut(context, self.as_value());

                let ret = reader.call(context, args);

                if ret.typ() == context.ListType {
                    let ret_list = ret.downcast::<Object<List>>().unwrap();
                    let first = ret_list.first(context);

                    if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                        return ret_list;
                    }
                }
            }

            let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
            ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, false).as_value());
            ret_list
        }
    }

    #[inline]
    pub fn collect(&mut self, context: &Context) -> Ptr<Object<List>> {
        let mut list = context.gc.new_object(context.ListType, List::new(context));

        loop {
            let ret = self.next(context);
            let first = ret.first(context);

            if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                list.push_back_mut(context, ret.last(context));
            } else {
                break;
            }
        }

        list
    }
}
