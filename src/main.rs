extern crate lisp;


use std::fs::File;
use std::io::prelude::*;


use lisp::lang::*;
use lisp::{eval, Ptr, Context};



pub fn lisp_to_string(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let mut string = String::new();

    loop {
        string.push_str(&args.first(context).to_string());
        args = args.pop(context);

        if *args.is_empty(context).value() {
            break;
        } else {
            string.push(',');
            string.push(' ');
        }
    }

    context.gc.new_object(context.StringType, string).as_value()
}

pub fn lisp_print(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let string = lisp_to_string(context, scope, args);
    println!("{}", string);
    context.nil_value.as_value()
}

pub fn lisp_uint_add(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UIntType && right.typ() == context.UIntType {
        let a = left.downcast::<Object<usize>>().unwrap();
        let b = right.downcast::<Object<usize>>().unwrap();
        context.gc.new_object(context.UIntType, a.value().wrapping_add(*b.value())).as_value()
    } else {
        context.gc.new_object(context.UIntType, 0usize).as_value()
    }
}

pub fn lisp_uint_eq(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UIntType && right.typ() == context.UIntType {
        let a = left.downcast::<Object<usize>>().unwrap();
        let b = right.downcast::<Object<usize>>().unwrap();
        context.gc.new_object(context.BooleanType, a.value() == b.value()).as_value()
    } else {
        context.gc.new_object(context.BooleanType, false).as_value()
    }
}

pub fn lisp_uint_sub(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UIntType && right.typ() == context.UIntType {
        let a = left.downcast::<Object<usize>>().unwrap();
        let b = right.downcast::<Object<usize>>().unwrap();
        context.gc.new_object(context.UIntType, a.value().wrapping_sub(*b.value())).as_value()
    } else {
        context.gc.new_object(context.UIntType, 0usize).as_value()
    }
}

pub fn lisp_uint_mul(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.UIntType && right.typ() == context.UIntType {
        let a = left.downcast::<Object<usize>>().unwrap();
        let b = right.downcast::<Object<usize>>().unwrap();
        context.gc.new_object(context.UIntType, a.value().wrapping_mul(*b.value())).as_value()
    } else {
        context.gc.new_object(context.UIntType, 0usize).as_value()
    }
}

fn main() {
    let mut context = Context::new();

    context.scope.set("uint_eq", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_uint_eq)).as_value());
    context.scope.set("uint_sub", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_uint_sub)).as_value());
    context.scope.set("uint_mul", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_uint_mul)).as_value());
    context.scope.set("uint_add", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_uint_add)).as_value());

    context.scope.set("print", context.gc.new_object(context.FunctionType,
        Function::new_rust(lisp_print)).as_value());

    let vec_a = context.gc.new_object(context.VectorType, Vector::new(&context));
    let vec_b = vec_a.push(&context, context.true_value.as_value());
    let vec_c = vec_b.push(&context, context.false_value.as_value());

    context.scope.set("vec_a", vec_a.as_value());
    context.scope.set("vec_b", vec_b.as_value());
    context.scope.set("vec_c", vec_c.as_value());

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
