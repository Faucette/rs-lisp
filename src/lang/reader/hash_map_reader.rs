use ::{Context, Ptr};
use ::lang::{Value, Object, Scope, List, HashMap};
use super::reader::Reader;


pub fn hash_map_reader(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.first(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == '{' {
        reader.read();

        let mut hash_map = context.gc.new_object(context.HashMapType, HashMap::new());

        loop {
            let ch = reader.peek(0);

            if ch == '}' {
                reader.read();
                break;
            } else {
                let key_ret_list = reader.next(context, scope);
                let first = key_ret_list.first(context);

                if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                    let value_ret_list = reader.next(context, scope);
                    let first = value_ret_list.first(context);

                    if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                        hash_map.set_mut(key_ret_list.last(context), value_ret_list.last(context));
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.true_value.as_value());
        ret_list.push_back_mut(context, hash_map.as_value());
        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.false_value.as_value());
        ret_list.as_value()
    }
}
