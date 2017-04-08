use super::super::super::super::utils::Ptr;
use super::super::super::value::Value;
use super::super::super::object::Object;
use super::super::list::List;
use super::super::primitives::Nil;
use super::reader::Reader;


pub fn list_reader(args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek().downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == '(' {
        reader.read();

        let mut list = List::new_empty();

        loop {
            let ch = reader.peek(0);

            if ch == ')' {
                reader.read();
                break;
            } else {
                if let Some(value) = reader.next() {
                    list.push_back_mut(value);
                } else {
                    break;
                }
            }
        }

        list.as_value()
    } else {
        Nil::new().as_value()
    }
}
