use ::Ptr;
use ::Context;
use ::lang::{Value, Object, List};
use super::reader::Reader;


pub fn list_reader(context: &Context, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == '(' {
        reader.read();

        let mut list = context.gc.new_object(context.ListType, List::new(context));

        loop {
            let ch = reader.peek(0);

            if ch == ')' {
                reader.read();
                break;
            } else {
                let ret_list = reader.next(context);
                let first = ret_list.first(context);

                if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                    list.push_back_mut(context, ret_list.last(context));
                } else {
                    break;
                }
            }
        }

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, true).as_value());
        ret_list.push_back_mut(context, list.as_value());
        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.gc.new_object(context.BooleanType, false).as_value());
        ret_list.as_value()
    }
}
