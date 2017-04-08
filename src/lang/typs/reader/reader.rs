use super::super::super::super::utils::Ptr;
use super::super::super::object::Object;
use super::super::super::value::Value;
use super::super::typs::{READER, NIL};
use super::super::function::Function;
use super::super::list::List;

use super::list_reader::list_reader;
use super::symbol_reader::symbol_reader;
use super::whitespace_reader::whitespace_reader;


pub struct Reader {
    readers: Vec<Ptr<Function>>,
    input: Vec<char>,
    index: usize,
    row: u64,
    col: u64,
}

unsafe impl Send for Reader {}
unsafe impl Sync for Reader {}

impl Reader {

    #[inline]
    pub fn new(input: Vec<char>) -> Ptr<Object<Self>> {
        let mut readers = Vec::new();

        readers.push(Function::new(list_reader));
        readers.push(Function::new(whitespace_reader));
        readers.push(Function::new(symbol_reader));

        Object::new(unsafe {READER} , Reader {
            readers: readers,
            input: input,
            index: 0usize,
            row: 1u64,
            col: 1u64,
        })
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
    pub fn next(&mut self) -> Option<Ptr<Value>> {
        if self.done() {
            None
        } else {
            let mut value = None;

            for reader in self.readers.iter() {
                let mut args = List::new_empty();
                args.push_back_mut(self.as_value());

                let ret = reader.call(args);

                if &**(ret.typ()) != unsafe {&**NIL} {
                    value = Some(ret);
                    break;
                }
            }

            value
        }
    }

    #[inline]
    pub fn collect(&mut self) -> Vec<Ptr<Value>> {
        let mut out = Vec::new();

        while let Some(value) = self.next() {
            out.push(value);
        }

        out
    }
}
