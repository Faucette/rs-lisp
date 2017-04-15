use ::{Ptr, Context};
use ::lang::{Value, Object, Scope, List, Vector};
use super::reader::Reader;


pub fn vector_reader(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.first(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == '[' {
        reader.read();

        let mut vector = context.gc.new_object(context.VectorType, Vector::new(context));

        loop {
            let ch = reader.peek(0);

            if ch == ']' {
                reader.read();
                break;
            } else {
                let ret_list = reader.next(context, scope);
                let first = ret_list.first(context);

                if first.typ() == context.BooleanType && first.downcast::<Object<bool>>().unwrap().value() == &true {
                    vector.push_mut(context, ret_list.last(context));
                } else {
                    break;
                }
            }
        }

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.true_value.as_value());
        ret_list.push_back_mut(context, vector.as_value());
        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.false_value.as_value());
        ret_list.as_value()
    }
}
