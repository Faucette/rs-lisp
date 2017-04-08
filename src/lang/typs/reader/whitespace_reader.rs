use super::super::super::super::utils::Ptr;
use super::super::super::value::Value;
use super::super::super::object::Object;
use super::super::list::List;
use super::super::primitives::{Symbol, Nil};
use super::reader::Reader;


pub fn whitespace_reader(args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek().downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch.is_whitespace() || ch == ',' {
        reader.read();

        while !reader.done() {
            let ch = reader.peek(0);

            if ch.is_whitespace() || ch == ',' {
                reader.read();
            } else {
                break;
            }
        }
    }

    Nil::new().as_value()
}
