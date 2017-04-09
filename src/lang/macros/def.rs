use collections::string::String;

use ::{Ptr, Context, eval};
use ::lang::{Value, Object, Scope, List, Keyword, Symbol};


#[inline]
pub fn def(context: &Context, mut scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
    let name: String = {
        let value = args.first(context);

        if value.typ() == context.KeywordType {
            args = args.pop(context);
            let keyword = value.downcast::<Object<Keyword>>().unwrap();
            (*keyword.value()).clone()
        } else if value.typ() == context.SymbolType {
            args = args.pop(context);
            let symbol = value.downcast::<Object<Symbol>>().unwrap();
            (*symbol.value()).clone()
        } else {
            panic!("invalid argument name given")
        }
    };

    let value = eval(context, scope, args.first(context));
    scope.set(&name, value);
    value
}
