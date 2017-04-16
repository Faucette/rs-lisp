use ::{Context, Ptr};
use ::lang::{Value, Object, Scope, List, Symbol};
use super::reader::Reader;


pub fn quote_reader(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.first(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);

    if ch == '`' {
        reader.read();

        let mut list = context.gc.new_object(context.ListType, List::new(context));

        list.push_back_mut(context,
            context.gc.new_object(context.SymbolType, Symbol::new("quote".into())).as_value());

        let ret_list = reader.next(context, scope);
        let value = ret_list.last(context);
        list.push_back_mut(context, value);

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.true_value.as_value());
        ret_list.push_back_mut(context, list.as_value());
        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.false_value.as_value());
        ret_list.as_value()
    }
}
