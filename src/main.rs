extern crate lambda;


use std::fs::File;
use std::io::prelude::*;

use lambda::lang::*;
use lambda::{eval, Ptr, Context};


pub fn lambda_to_string(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
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

pub fn lambda_print(context: &Context, scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let string = lambda_to_string(context, scope, args);
    println!("{}", string);
    context.nil_value.as_value()
}

pub fn lambda_number_add(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.NumberType && right.typ() == context.NumberType {
        let a = left.downcast::<Object<Number>>().unwrap();
        let b = right.downcast::<Object<Number>>().unwrap();
        context.gc.new_object(context.NumberType, Number::from(a.value() + b.value())).as_value()
    } else {
        context.gc.new_object(context.NumberType, Number::from(0usize)).as_value()
    }
}

pub fn lambda_number_eq(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.NumberType && right.typ() == context.NumberType {
        let a = left.downcast::<Object<Number>>().unwrap();
        let b = right.downcast::<Object<Number>>().unwrap();
        context.gc.new_object(context.BooleanType, a.value() == b.value()).as_value()
    } else {
        context.gc.new_object(context.BooleanType, false).as_value()
    }
}

pub fn lambda_number_sub(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.NumberType && right.typ() == context.NumberType {
        let a = left.downcast::<Object<Number>>().unwrap();
        let b = right.downcast::<Object<Number>>().unwrap();
        context.gc.new_object(context.NumberType, Number::from(a.value() - b.value())).as_value()
    } else {
        context.gc.new_object(context.NumberType, Number::from(0usize)).as_value()
    }
}

pub fn lambda_number_mul(context: &Context, _: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let left = args.first(context);
    args = args.pop(context);
    let right = args.first(context);

    if left.typ() == context.NumberType && right.typ() == context.NumberType {
        let a = left.downcast::<Object<Number>>().unwrap();
        let b = right.downcast::<Object<Number>>().unwrap();
        context.gc.new_object(context.NumberType, Number::from(a.value() * b.value())).as_value()
    } else {
        context.gc.new_object(context.NumberType, Number::from(0usize)).as_value()
    }
}

fn main() {
    let context = Context::new();

    context.scope.set(&context, context.symbol("number_eq").as_value(),
        context.gc.new_object(context.FunctionType, Function::new_rust(lambda_number_eq)).as_value());
    context.scope.set(&context, context.symbol("number_sub").as_value(),
        context.gc.new_object(context.FunctionType, Function::new_rust(lambda_number_sub)).as_value());
    context.scope.set(&context, context.symbol("number_mul").as_value(),
        context.gc.new_object(context.FunctionType, Function::new_rust(lambda_number_mul)).as_value());
    context.scope.set(&context, context.symbol("number_add").as_value(),
        context.gc.new_object(context.FunctionType, Function::new_rust(lambda_number_add)).as_value());
    context.scope.set(&context, context.symbol("print").as_value(),
        context.gc.new_object(context.FunctionType, Function::new_rust(lambda_print)).as_value());

    let mut file = File::open("tests/fac.ll").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let input = contents.chars().collect();

    let mut reader = context.gc.new_object(context.ReaderType, Reader::new(&context, input));
    let mut ast = reader.collect(&context, context.scope);

    while !ast.is_empty(&context).value() {
        let result = eval(&context, context.scope, ast.first(&context));
        println!("{:?}", result);
        ast = ast.pop(&context);
    }

    println!("\nTotel: {:?} bytes\n", context.gc.total());
}
