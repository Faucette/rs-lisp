extern crate lisp;


use std::fs::File;
use std::io::prelude::*;


use lisp::lang::*;
use lisp::{eval, Ptr, Context};



pub fn add_uint64(context: &Context, scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UInt64Type && right.typ() == context.UInt64Type {
        let a = left.downcast::<Object<u64>>().unwrap();
        let b = right.downcast::<Object<u64>>().unwrap();
        context.gc.new_object(context.UInt64Type, a.value() + b.value()).as_value()
    } else {
        context.gc.new_object(context.UInt64Type, 0u64).as_value()
    }
}

fn main() {
    let mut context = Context::new();
    context.scope.set("add_uint64", context.gc.new_object(context.FunctionType, Function::new_rust(add_uint64)).as_value());

    let mut file = File::open("tests/test.s").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = contents.chars().collect();

    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(&context, input));
    let mut values = reader.collect(&context, context.scope);
    let mut result = context.nil_value.as_value();

    println!("\nAST: {:?}\n", values);

    while !values.is_empty(&context).value() {
        result = eval(&context, context.scope, values.first(&context));
        values = values.pop(&context);
    }

    println!("\nTotel: {:?} bytes\n", context.gc.total());
}
