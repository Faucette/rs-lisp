extern crate lisp;


use std::fs::File;
use std::io::prelude::*;


use lisp::lang::*;
use lisp::{eval, Ptr, Context};



pub fn lisp_print(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {

    loop {
        print!("{:?}", args.first(context));
        args = args.pop(context);

        if *args.is_empty(context).value() {
            break;
        } else {
            print!(", ");
        }
    }
    println!("");

    context.nil_value.as_value()
}

pub fn lisp_add_uint64(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
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

pub fn lisp_int_eq(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UInt64Type && right.typ() == context.UInt64Type {
        let a = left.downcast::<Object<u64>>().unwrap();
        let b = right.downcast::<Object<u64>>().unwrap();
        context.gc.new_object(context.BooleanType, a.value() == b.value()).as_value()
    } else {
        context.gc.new_object(context.BooleanType, false).as_value()
    }
}

pub fn lisp_int_sub(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UInt64Type && right.typ() == context.UInt64Type {
        let a = left.downcast::<Object<u64>>().unwrap();
        let b = right.downcast::<Object<u64>>().unwrap();
        context.gc.new_object(context.UInt64Type, a.value() - b.value()).as_value()
    } else {
        context.gc.new_object(context.UInt64Type, 0u64).as_value()
    }
}

pub fn lisp_int_mul(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UInt64Type && right.typ() == context.UInt64Type {
        let a = left.downcast::<Object<u64>>().unwrap();
        let b = right.downcast::<Object<u64>>().unwrap();
        context.gc.new_object(context.UInt64Type, a.value() * b.value()).as_value()
    } else {
        context.gc.new_object(context.UInt64Type, 0u64).as_value()
    }
}

fn main() {
    let mut context = Context::new();

    context.scope.set("add_uint64", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_add_uint64)).as_value());

    context.scope.set("int_eq", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_int_eq)).as_value());
    context.scope.set("int_sub", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_int_sub)).as_value());
    context.scope.set("int_mul", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_int_mul)).as_value());

    context.scope.set("print", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_print)).as_value());

    let vec_a = context.gc.new_object(context.VectorType, Vector::new(&context));
    let vec_b = vec_a.push(&context, context.true_value.as_value());

    context.scope.set("vec_a", vec_a.as_value());
    context.scope.set("vec_b", vec_b.as_value());

    let mut file = File::open("tests/test.s").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = contents.chars().collect();

    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(&context, input));
    let mut values = reader.collect(&context, context.scope);

    println!("\nAST: {:?}\n", values);

    while !values.is_empty(&context).value() {
        let result = eval(&context, context.scope, values.first(&context));
        println!("{:?}", result);
        values = values.pop(&context);
    }

    println!("\nTotel: {:?} bytes\n", context.gc.total());
}
