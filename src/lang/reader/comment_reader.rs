use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List};
use super::reader::Reader;


pub fn comment_reader(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.first(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == ';' {
        reader.read();

        while !reader.done() {
            let ch = reader.peek(0);

            if ch != '\n' {
                reader.read();
            } else {
                break;
            }
        }
    }

    let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
    ret_list.push_back_mut(context, context.false_value.as_value());
    ret_list.as_value()
}
