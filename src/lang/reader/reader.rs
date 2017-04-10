use core::fmt;

use collection_traits::*;
use vector::Vector;

use ::Ptr;
use ::Context;
use ::lang::{Value, Object, Callable, Function, Scope, List};

use super::list_reader::list_reader;
use super::number_reader::number_reader;
use super::symbol_reader::symbol_reader;
use super::quote_reader::quote_reader;
use super::whitespace_reader::whitespace_reader;


pub struct Reader {
    readers: Vector<Ptr<Object<Function>>>,
    input: Vector<char>,
    index: usize,
    row: u64,
    col: u64,
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl Reader {

    #[inline]
    pub fn new(context: &Context, input: Vector<char>) -> Self {
        let mut readers = Vector::new();

        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(whitespace_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(list_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(number_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(quote_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(symbol_reader)));

        Reader {
            readers: readers,
            input: input,
            index: 0usize,
            row: 1u64,
            col: 1u64,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, _args: Ptr<Object<List>>) -> Ptr<Value> {
        context.gc.new_object(context.ReaderType, Self::new(&context, "".chars().collect())).as_value()
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
    pub fn next(&mut self, context: &Context, scope: Ptr<Object<Scope>>) -> Ptr<Object<List>> {
        if self.done() {
            let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
            ret_list.push_back_mut(context, context.false_value.as_value());
            ret_list
        } else {
            for reader in self.readers.iter() {
                let mut args = context.gc.new_object(context.ListType, List::new(context));
                args.push_back_mut(context, self.as_value());

                let ret = reader.call(context, scope, args);

                if ret.typ() == context.ListType {
                    let ret_list = ret.downcast::<Object<List>>().unwrap();
                    let first = ret_list.first(context);

                    if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                        return ret_list;
                    }
                }
            }

            let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
            ret_list.push_back_mut(context, context.false_value.as_value());
            ret_list
        }
    }

    #[inline]
    pub fn collect(&mut self, context: &Context, scope: Ptr<Object<Scope>>) -> Ptr<Object<List>> {
        let mut list = context.gc.new_object(context.ListType, List::new(context));

        loop {
            let ret = self.next(context, scope);
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

impl fmt::Debug for Reader {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%Reader{{}}")
    }
}
