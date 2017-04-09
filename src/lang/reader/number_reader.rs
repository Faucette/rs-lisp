use collections::string::String;

use ::Ptr;
use ::Context;
use ::lang::{Value, Object, Scope, List};
use super::reader::Reader;


pub fn number_reader(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut reader = args.peek(context).downcast::<Object<Reader>>().unwrap();

    let ch = reader.peek(0);
    let is_negative = ch == '-';
    let next_ch = if reader.can_read(1) {reader.peek(1)} else {'\0'};

    if ch.is_numeric() || (is_negative && next_ch.is_numeric()) {
        reader.read();

        let mut string = String::new();
        let mut read_dot = false;

        string.push(ch);

        if is_negative {
            reader.read();
            string.push(next_ch);
        }

        while !reader.done() {
            let ch = reader.peek(0);

            if ch == '.' && !read_dot {
                reader.read();
                string.push(ch);
                read_dot = true;
            } else if ch.is_numeric() {
                reader.read();
                string.push(ch);
            } else {
                break;
            }
        }

        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.true_value.as_value());

        if read_dot {
            ret_list.push_back_mut(context,
                context.gc.new_object(context.Float64Type, string.parse::<f64>().unwrap()).as_value());
        } else if is_negative {
            ret_list.push_back_mut(context,
                context.gc.new_object(context.Int64Type, string.parse::<i64>().unwrap()).as_value());
        } else {
            ret_list.push_back_mut(context,
                context.gc.new_object(context.UInt64Type, string.parse::<u64>().unwrap()).as_value());
        }

        ret_list.as_value()
    } else {
        let mut ret_list = context.gc.new_object(context.ListType, List::new(context));
        ret_list.push_back_mut(context, context.false_value.as_value());
        ret_list.as_value()
    }
}
