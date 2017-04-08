use ::Ptr;
use ::Context;
use ::lang::{Value, Object, List};
use super::reader::Reader;


pub fn whitespace_reader(context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek(context).downcast::<Object<Reader>>().unwrap();

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

    let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
    ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, false).as_value());
    ret_list.as_value()
}
