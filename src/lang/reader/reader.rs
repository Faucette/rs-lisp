use collections::string::String;

use core::fmt;

use collection_traits::*;
use vector::Vector;

use ::{Context, LHash, Ptr};
use ::lang::{Value, Object, Function, Scope, List};

use super::comment_reader::comment_reader;
use super::hash_map_reader::hash_map_reader;
use super::identifier_reader::identifier_reader;
use super::list_reader::list_reader;
use super::number_reader::number_reader;
use super::quote_reader::quote_reader;
use super::quoted_reader::quoted_reader;
use super::vector_reader::vector_reader;
use super::whitespace_reader::whitespace_reader;


#[derive(Hash)]
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
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(comment_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(list_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(vector_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(hash_map_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(number_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(quote_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(quoted_reader)));
        readers.push(context.gc.new_object(context.FunctionType, Function::new_rust(identifier_reader)));

        Reader {
            readers: readers,
            input: input,
            index: 0usize,
            row: 1u64,
            col: 1u64,
        }
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
        let first = args.first(context);

        let reader = if first.typ() == context.StringType {
            let string = first.downcast::<Object<String>>().unwrap();
            Reader::new(context, string.chars().collect())
        } else {
            Reader::new(context, "".chars().collect())
        };

        context.gc.new_object(context.ReaderType, reader).as_value()
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

                let ret = match &***reader {
                    &Function::Rust(ref fn_ptr) => (fn_ptr)(context, scope, args),
                    _ => panic!("can not call non rust readers right now!"),
                };

                if ret.typ() == context.ListType {
                    let ret_list = ret.downcast::<Object<List>>().unwrap();
                    let first = ret_list.first(context);

                    if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                        return ret_list;
                    }
                }

                if self.done() {
                    break;
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

impl fmt::Display for Reader {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string: String = self.input.iter().collect();
        write!(f, "(Reader {:?})", string)
    }
}

impl fmt::Debug for Reader {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
