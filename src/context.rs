use collections::string::String;

use core::mem;
use core::sync::atomic::{AtomicPtr, Ordering};

use ::{Ptr, Gc};
use ::lang::{
    Value, Object, TypeBuilder,
    Scope, Number, Keyword, Symbol, List, Vector, HashMap, Function, Nil, Reader, Type
};


pub static DEFAULT_SCOPE: &'static str = "user";


#[allow(non_snake_case)]
pub struct Context {
    pub TypeType: Ptr<Object<Type>>,

    pub FunctionType: Ptr<Object<Type>>,
    pub MacroType: Ptr<Object<Type>>,
    pub ScopeType: Ptr<Object<Type>>,
    pub NilType: Ptr<Object<Type>>,

    pub ReaderType: Ptr<Object<Type>>,

    pub ListType: Ptr<Object<Type>>,
    pub VectorType: Ptr<Object<Type>>,
    pub HashMapType: Ptr<Object<Type>>,

    pub SymbolType: Ptr<Object<Type>>,
    pub KeywordType: Ptr<Object<Type>>,
    pub StringType: Ptr<Object<Type>>,

    pub BooleanType: Ptr<Object<Type>>,

    pub NumberType: Ptr<Object<Type>>,

    pub true_value: Ptr<Object<bool>>,
    pub false_value: Ptr<Object<bool>>,
    pub nil_value: Ptr<Object<Nil>>,

    namespaces: AtomicPtr<Object<HashMap>>,
    pub scope: Ptr<Object<Scope>>,
    pub gc: Gc,
}

impl Context {

    #[allow(non_snake_case)]
    pub fn new() -> Self {
        let gc = Gc::new();

        let mut TypeType = gc.new_null_typ_object(TypeBuilder::new("Type").build());
        TypeType.typ = TypeType;

        let mut FunctionType = gc.new_object(TypeType, TypeBuilder::new("Function")
            .size(mem::size_of::<Function>())
            .build());

        FunctionType.value.constructor = Some(gc.new_object(FunctionType,
            Function::new_rust(Function::constructor)));

        let MacroType = gc.new_object(TypeType, TypeBuilder::new("Macro")
            .size(mem::size_of::<Function>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Function::macro_constructor)))
            .build());

        let ScopeType = gc.new_object(TypeType, TypeBuilder::new("Scope")
            .size(mem::size_of::<Scope>())
            .build());
        let NilType = gc.new_object(TypeType, TypeBuilder::new("Nil")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Nil::constructor)))
            .build());

        let ReaderType = gc.new_object(TypeType, TypeBuilder::new("Reader")
            .size(mem::size_of::<Reader>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Reader::constructor)))
            .build());

        let ListType = gc.new_object(TypeType, TypeBuilder::new("List")
            .size(mem::size_of::<List>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(List::constructor)))
            .build());
        let VectorType = gc.new_object(TypeType, TypeBuilder::new("Vector")
            .size(mem::size_of::<Vector>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Vector::constructor)))
            .build());
        let HashMapType = gc.new_object(TypeType, TypeBuilder::new("HashMap")
            .size(mem::size_of::<HashMap>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(HashMap::constructor)))
            .build());

        let SymbolType = gc.new_object(TypeType, TypeBuilder::new("Symbol")
            .size(mem::size_of::<String>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Symbol::constructor)))
            .build());
        let KeywordType = gc.new_object(TypeType, TypeBuilder::new("Keyword")
            .size(mem::size_of::<String>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Keyword::constructor)))
            .build());
        let StringType = gc.new_object(TypeType, TypeBuilder::new("String")
            .size(mem::size_of::<String>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(String_constructor)))
            .build());

        let BooleanType = gc.new_object(TypeType, TypeBuilder::new("Boolean")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Boolean_constructor)))
            .size(mem::size_of::<bool>()).build());

        let NumberType = gc.new_object(TypeType, TypeBuilder::new("Number")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Number::constructor)))
            .size(mem::size_of::<Number>()).build());

        let true_value = gc.new_object(BooleanType, true);
        let false_value = gc.new_object(BooleanType, false);
        let nil_value = gc.new_object(NilType, Nil::new());

        let mut raw_namespsaces = gc.new_object(HashMapType, HashMap::new());

        let mut scope = gc.new_object(ScopeType,
            Scope::from_mappings(
                Some(gc.new_object(SymbolType, Symbol::new(DEFAULT_SCOPE.into()))),
                None,
                gc.new_object(HashMapType, HashMap::new())
            ));

        raw_namespsaces.set_mut(scope.name.unwrap().as_value(), scope.as_value());

        let namespaces = unsafe {
            AtomicPtr::new(raw_namespsaces.as_ptr())
        };

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Type".into())).as_value(), TypeType.as_value());

        // scope.set_mut(gc.new_object(SymbolType, Symbol::new("Scope".into())).as_value(), ScopeType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Function".into())).as_value(), FunctionType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Macro".into())).as_value(), MacroType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Nil".into())).as_value(), NilType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Reader".into())).as_value(), ReaderType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("List".into())).as_value(), ListType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Vector".into())).as_value(), VectorType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Symbol".into())).as_value(), SymbolType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Keyword".into())).as_value(), KeywordType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("String".into())).as_value(), StringType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Boolean".into())).as_value(), BooleanType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Number".into())).as_value(), NumberType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("true".into())).as_value(), true_value.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("false".into())).as_value(), false_value.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("nil".into())).as_value(), nil_value.as_value());

        Context {
            TypeType: TypeType,

            FunctionType: FunctionType,
            MacroType: MacroType,
            ScopeType: ScopeType,
            NilType: NilType,

            ReaderType: ReaderType,

            ListType: ListType,
            VectorType: VectorType,
            HashMapType: HashMapType,

            SymbolType: SymbolType,
            KeywordType: KeywordType,
            StringType: StringType,

            BooleanType: BooleanType,

            NumberType: NumberType,

            nil_value: nil_value,
            true_value: true_value,
            false_value: false_value,

            namespaces: namespaces,
            scope: scope,
            gc: gc,
        }
    }

    #[inline(always)]
    pub fn string(&self, name: &str) -> Ptr<Object<String>> {
        self.gc.new_object(self.StringType, String::from(name))
    }
    #[inline(always)]
    pub fn symbol(&self, name: &str) -> Ptr<Object<Symbol>> {
        self.gc.new_object(self.SymbolType, Symbol::new(String::from(name)))
    }
    #[inline(always)]
    pub fn keyword(&self, name: &str) -> Ptr<Object<Keyword>> {
        self.gc.new_object(self.KeywordType, Keyword::new(String::from(name)))
    }

    #[inline(always)]
    fn namespaces_ptr(&self) -> Ptr<Object<HashMap>> {
        unsafe {
            Ptr::from_ptr(self.namespaces.load(Ordering::Relaxed))
        }
    }
    #[inline(always)]
    fn namespaces_swap(&self, hash_map: Ptr<Object<HashMap>>) {
        self.namespaces.store(unsafe {hash_map.as_ptr()}, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn namespace(&self, parent: Option<Ptr<Object<Scope>>>, name: Ptr<Object<Symbol>>) -> Ptr<Object<Scope>> {
        let hash_map = self.namespaces_ptr();
        let name_value = name.as_value();

        match (&**hash_map).get(name_value) {
            Some(scope) => scope.downcast::<Object<Scope>>().unwrap(),
            None => {
                let scope = self.gc.new_object(self.ScopeType,
                    Scope::from_mappings(
                        Some(name),
                        parent,
                        self.gc.new_object(self.HashMapType, HashMap::new())
                    ));

                let new_hash_map = self.namespaces_ptr().set(self, name_value, scope.as_value());
                self.namespaces_swap(new_hash_map);

                scope
            }
        }
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn String_constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let value = args.first(context);

    if value.typ() == context.StringType {
        value
    } else if value.typ() == context.SymbolType {

        let symbol = value.downcast::<Object<Symbol>>().unwrap();
        context.gc.new_object(context.StringType, (*symbol.value()).clone()).as_value()

    } else if value.typ() == context.KeywordType {

        let keyword = value.downcast::<Object<Keyword>>().unwrap();
        context.gc.new_object(context.StringType, (*keyword.value()).clone()).as_value()

    } else {

        context.gc.new_object(context.StringType, String::new()).as_value()
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn Boolean_constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let value = args.first(context);

    if value.typ() == context.BooleanType {
        value
    } else {
        context.false_value.as_value()
    }
}
