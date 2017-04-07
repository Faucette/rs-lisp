use super::super::super::utils::Ptr;
use super::super::typs::Function;
use super::super::value::Value;


pub struct Reader {
    readers: Vec<Ptr<Function>>,
    input: Vec<char>,
    index: usize,
}

impl<'a> From<&'a str> for Reader {

    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Reader {
            readers: Vec::new(),
            input: value.chars().collect(),
            index: 0usize,
        }
    }
}

impl Iterator for Reader {
    type Item = Ptr<Value>;


    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
