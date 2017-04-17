use collections::string::String;

use core::mem;
use core::sync::atomic::{AtomicPtr, Ordering};

use ::{Ptr, Gc};
use ::lang::{
    Value, Object, TypeBuilder,
    Scope, Keyword, Symbol, List, Vector, HashMap, Function, Nil, Reader, Type
};


pub static DEFAULT_SCOPE: &'static str = "user";


#[allow(non_snake_case)]
pub struct Context {
    pub AnyType: Ptr<Object<Type>>,
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

    pub NumberType: Ptr<Object<Type>>,
    pub RealType: Ptr<Object<Type>>,
    pub FloatType: Ptr<Object<Type>>,
    pub IntegerType: Ptr<Object<Type>>,
    pub SignedType: Ptr<Object<Type>>,
    pub UnsignedType: Ptr<Object<Type>>,

    pub BooleanType: Ptr<Object<Type>>,
    pub CharType: Ptr<Object<Type>>,

    pub IntType: Ptr<Object<Type>>,
    pub Int8Type: Ptr<Object<Type>>,
    pub Int16Type: Ptr<Object<Type>>,
    pub Int32Type: Ptr<Object<Type>>,
    pub Int64Type: Ptr<Object<Type>>,

    pub UIntType: Ptr<Object<Type>>,
    pub UInt8Type: Ptr<Object<Type>>,
    pub UInt16Type: Ptr<Object<Type>>,
    pub UInt32Type: Ptr<Object<Type>>,
    pub UInt64Type: Ptr<Object<Type>>,

    pub Float32Type: Ptr<Object<Type>>,
    pub Float64Type: Ptr<Object<Type>>,

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

        let mut AnyType = gc.new_object(TypeType, TypeBuilder::new("Any")
            .is_abstract().build());

        AnyType.value.supr = Some(AnyType);
        TypeType.value.supr = Some(AnyType);

        let mut FunctionType = gc.new_object(TypeType, TypeBuilder::new("Function")
            .size(mem::size_of::<Function>())
            .supr(AnyType).build());

        FunctionType.value.constructor = Some(gc.new_object(FunctionType,
            Function::new_rust(Function::constructor)));

        let MacroType = gc.new_object(TypeType, TypeBuilder::new("Macro")
            .size(mem::size_of::<Function>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Function::macro_constructor)))
            .supr(AnyType).build());

        TypeType.value.constructor = Some(gc.new_object(FunctionType,
            Function::new_rust(Type::constructor)));

        let ScopeType = gc.new_object(TypeType, TypeBuilder::new("Scope")
            .size(mem::size_of::<Scope>())
            .supr(AnyType).build());
        let NilType = gc.new_object(TypeType, TypeBuilder::new("Nil")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Nil::constructor)))
            .supr(AnyType).build());

        let ReaderType = gc.new_object(TypeType, TypeBuilder::new("Reader")
            .size(mem::size_of::<Reader>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Reader::constructor)))
            .supr(AnyType).build());

        let ListType = gc.new_object(TypeType, TypeBuilder::new("List")
            .supr(AnyType)
            .size(mem::size_of::<List>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(List::constructor)))
            .build());
        let VectorType = gc.new_object(TypeType, TypeBuilder::new("Vector")
            .supr(AnyType)
            .size(mem::size_of::<Vector>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Vector::constructor)))
            .build());
        let HashMapType = gc.new_object(TypeType, TypeBuilder::new("HashMap")
            .supr(AnyType)
            .size(mem::size_of::<HashMap>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(HashMap::constructor)))
            .build());

        let SymbolType = gc.new_object(TypeType, TypeBuilder::new("Symbol")
            .supr(AnyType)
            .size(mem::size_of::<String>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Symbol::constructor)))
            .build());
        let KeywordType = gc.new_object(TypeType, TypeBuilder::new("Keyword")
            .supr(AnyType)
            .size(mem::size_of::<String>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(Keyword::constructor)))
            .build());
        let StringType = gc.new_object(TypeType, TypeBuilder::new("String")
            .supr(AnyType)
            .size(mem::size_of::<String>())
            .constructor(gc.new_object(FunctionType, Function::new_rust(String_constructor)))
            .build());

        let NumberType = gc.new_object(TypeType, TypeBuilder::new("Number")
            .supr(AnyType).is_abstract().build());

        let RealType = gc.new_object(TypeType, TypeBuilder::new("Real")
            .supr(NumberType).is_abstract().build());

        let FloatType = gc.new_object(TypeType, TypeBuilder::new("Float")
            .supr(RealType).is_abstract().build());

        let IntegerType = gc.new_object(TypeType, TypeBuilder::new("Integer")
            .supr(RealType).is_abstract().build());
        let SignedType = gc.new_object(TypeType, TypeBuilder::new("Signed")
            .supr(IntegerType).is_abstract().build());
        let UnsignedType = gc.new_object(TypeType, TypeBuilder::new("Unsigned")
            .supr(IntegerType).is_abstract().build());

        let BooleanType = gc.new_object(TypeType, TypeBuilder::new("Boolean")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Boolean_constructor)))
            .supr(IntegerType).size(mem::size_of::<bool>()).is_bits().build());
        let CharType = gc.new_object(TypeType, TypeBuilder::new("Char")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Char_constructor)))
            .supr(AnyType).size(mem::size_of::<char>()).is_bits().build());

        let IntType = gc.new_object(TypeType, TypeBuilder::new("Int")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Int_constructor)))
            .supr(SignedType).size(mem::size_of::<isize>()).is_bits().build());
        let Int8Type = gc.new_object(TypeType, TypeBuilder::new("Int8")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Int8_constructor)))
            .supr(SignedType).size(mem::size_of::<i8>()).is_bits().build());
        let Int16Type = gc.new_object(TypeType, TypeBuilder::new("Int16")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Int16_constructor)))
            .supr(SignedType).size(mem::size_of::<i16>()).is_bits().build());
        let Int32Type = gc.new_object(TypeType, TypeBuilder::new("Int32")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Int32_constructor)))
            .supr(SignedType).size(mem::size_of::<i32>()).is_bits().build());
        let Int64Type = gc.new_object(TypeType, TypeBuilder::new("Int64")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Int64_constructor)))
            .supr(SignedType).size(mem::size_of::<i64>()).is_bits().build());

        let UIntType = gc.new_object(TypeType, TypeBuilder::new("UInt")
            .constructor(gc.new_object(FunctionType, Function::new_rust(UInt_constructor)))
            .supr(UnsignedType).size(mem::size_of::<usize>()).is_bits().build());
        let UInt8Type = gc.new_object(TypeType, TypeBuilder::new("UInt8")
            .constructor(gc.new_object(FunctionType, Function::new_rust(UInt8_constructor)))
            .supr(UnsignedType).size(mem::size_of::<u8>()).is_bits().build());
        let UInt16Type = gc.new_object(TypeType, TypeBuilder::new("UInt16")
            .constructor(gc.new_object(FunctionType, Function::new_rust(UInt16_constructor)))
            .supr(UnsignedType).size(mem::size_of::<u16>()).is_bits().build());
        let UInt32Type = gc.new_object(TypeType, TypeBuilder::new("UInt32")
            .constructor(gc.new_object(FunctionType, Function::new_rust(UInt32_constructor)))
            .supr(UnsignedType).size(mem::size_of::<u32>()).is_bits().build());
        let UInt64Type = gc.new_object(TypeType, TypeBuilder::new("UInt64")
            .constructor(gc.new_object(FunctionType, Function::new_rust(UInt64_constructor)))
            .supr(UnsignedType).size(mem::size_of::<u64>()).is_bits().build());

        let Float32Type = gc.new_object(TypeType, TypeBuilder::new("Float32")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Float32_constructor)))
            .supr(FloatType).size(mem::size_of::<f32>()).is_bits().build());
        let Float64Type = gc.new_object(TypeType, TypeBuilder::new("Float64")
            .constructor(gc.new_object(FunctionType, Function::new_rust(Float64_constructor)))
            .supr(FloatType).size(mem::size_of::<f64>()).is_bits().build());

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
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Any".into())).as_value(), AnyType.as_value());

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

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Number".into())).as_value(), NumberType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Real".into())).as_value(), RealType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Float".into())).as_value(), FloatType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Integer".into())).as_value(), IntegerType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Signed".into())).as_value(), SignedType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Unsigned".into())).as_value(), UnsignedType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Boolean".into())).as_value(), BooleanType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Char".into())).as_value(), CharType.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Int".into())).as_value(), IntType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Int8".into())).as_value(), Int8Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Int16".into())).as_value(), Int16Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Int32".into())).as_value(), Int32Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Int64".into())).as_value(), Int64Type.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("UInt".into())).as_value(), UIntType.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("UInt8".into())).as_value(), UInt8Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("UInt16".into())).as_value(), UInt16Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("UInt32".into())).as_value(), UInt32Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("UInt64".into())).as_value(), UInt64Type.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Float32".into())).as_value(), Float32Type.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("Float64".into())).as_value(), Float64Type.as_value());

        scope.set_mut(gc.new_object(SymbolType, Symbol::new("true".into())).as_value(), true_value.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("false".into())).as_value(), false_value.as_value());
        scope.set_mut(gc.new_object(SymbolType, Symbol::new("nil".into())).as_value(), nil_value.as_value());

        Context {
            AnyType: AnyType,
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

            NumberType: NumberType,
            RealType: RealType,
            FloatType: FloatType,
            IntegerType: IntegerType,
            SignedType: SignedType,
            UnsignedType: UnsignedType,

            BooleanType: BooleanType,
            CharType: CharType,

            IntType: IntType,
            Int8Type: Int8Type,
            Int16Type: Int16Type,
            Int32Type: Int32Type,
            Int64Type: Int64Type,

            UIntType: UIntType,
            UInt8Type: UInt8Type,
            UInt16Type: UInt16Type,
            UInt32Type: UInt32Type,
            UInt64Type: UInt64Type,

            Float32Type: Float32Type,
            Float64Type: Float64Type,

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

#[allow(non_snake_case)]
#[inline]
pub fn Char_constructor(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
    let value = args.first(context);

    if value.typ() == context.CharType {
        value
    } else {
        context.gc.new_object(context.CharType, '\0').as_value()
    } // TODO add cast from int to char
}

macro_rules! create_number_constructor {
    ($name:ident, $default:expr) => (
        #[allow(non_snake_case)]
        #[inline]
        pub fn $name(context: &Context, _scope: Ptr<Object<Scope>>, args: Ptr<Object<List>>) -> Ptr<Value> {
            let value = args.first(context);

            if value.typ().instance_of(&**context.NumberType) {
                context.gc.new_object(context.Int8Type, $default).as_value() // TODO add number casting
            } else {
                context.gc.new_object(context.Int8Type, $default).as_value()
            }
        }
    );
}

create_number_constructor!(Int_constructor, 0usize);
create_number_constructor!(Int8_constructor, 0u8);
create_number_constructor!(Int16_constructor, 0u16);
create_number_constructor!(Int32_constructor, 0u32);
create_number_constructor!(Int64_constructor, 0u64);

create_number_constructor!(UInt_constructor, 0isize);
create_number_constructor!(UInt8_constructor, 0i8);
create_number_constructor!(UInt16_constructor, 0i16);
create_number_constructor!(UInt32_constructor, 0i32);
create_number_constructor!(UInt64_constructor, 0i64);

create_number_constructor!(Float32_constructor, 0f32);
create_number_constructor!(Float64_constructor, 0f64);
