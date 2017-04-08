use super::super::super::super::utils::Ptr;
use super::super::super::value::Value;
use super::super::super::object::Object;
use super::super::list::List;
use super::super::primitives::{Symbol, Keyword, Nil, Boolean_new};
use super::reader::Reader;


pub fn symbol_reader(args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek().downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if !ch.is_whitespace() && ch != ',' {
        let first = ch;

        reader.read();

        let mut string = String::new();

        if first != ':' {
            string.push(ch);
        }

        while !reader.done() {
            let ch = reader.peek(0);

            if ch.is_whitespace() || ch == ',' || ch == ')' {
                break;
            } else {
                reader.read();
                string.push(ch);
            }
        }

        if first == ':' {
            Keyword::new(string).as_value()
        } else {
            match string.as_str() {
                "true" => Boolean_new(true).as_value(),
                "false" => Boolean_new(true).as_value(),
                "nil" => Nil::new().as_value(),
                _ => Symbol::new(string).as_value(),
            }
        }
    } else {
        Nil::new().as_value()
    }
}
