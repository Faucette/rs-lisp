use collections::string::String;

use core::mem;

use collection_traits::*;
use hash_map::HashMap;

use ::Ptr;
use ::Gc;
use ::lang::{Object, Symbol, Scope, List, Function, Reader, Type, TypeBuilder};


#[allow(non_snake_case)]
pub struct Context {
    pub AnyType: Ptr<Object<Type>>,
    pub TypeType: Ptr<Object<Type>>,

    pub ScopeType: Ptr<Object<Type>>,
    pub FunctionType: Ptr<Object<Type>>,
    pub NilType: Ptr<Object<Type>>,

    pub ReaderType: Ptr<Object<Type>>,

    pub ListType: Ptr<Object<Type>>,

    pub SymbolType: Ptr<Object<Type>>,
    pub KeywordType: Ptr<Object<Type>>,

    pub NumberType: Ptr<Object<Type>>,
    pub RealType: Ptr<Object<Type>>,
    pub FloatType: Ptr<Object<Type>>,
    pub IntegerType: Ptr<Object<Type>>,
    pub SignedType: Ptr<Object<Type>>,
    pub UnsignedType: Ptr<Object<Type>>,

    pub BooleanType: Ptr<Object<Type>>,
    pub CharType: Ptr<Object<Type>>,

    pub Int8Type: Ptr<Object<Type>>,
    pub Int16Type: Ptr<Object<Type>>,
    pub Int32Type: Ptr<Object<Type>>,
    pub Int64Type: Ptr<Object<Type>>,

    pub UInt8Type: Ptr<Object<Type>>,
    pub UInt16Type: Ptr<Object<Type>>,
    pub UInt32Type: Ptr<Object<Type>>,
    pub UInt64Type: Ptr<Object<Type>>,

    pub Float32Type: Ptr<Object<Type>>,
    pub Float64Type: Ptr<Object<Type>>,

    pub namespaces: HashMap<String, Ptr<Object<Scope>>>,

    pub gc: Gc,
}

impl Context {

    #[allow(non_snake_case)]
    pub fn new() -> Self {
        let gc = Gc::new();

        let mut TypeType = gc.new_null_typ_object(
            TypeBuilder::new("Type").is_abstract().build()
        );
        TypeType.typ = TypeType;

        let mut AnyType = gc.new_object(TypeType, TypeBuilder::new("Any")
            .is_abstract().build());

        AnyType.value.supr = Some(AnyType);
        TypeType.value.supr = Some(AnyType);

        let ScopeType = gc.new_object(TypeType, TypeBuilder::new("Scope")
            .size(mem::size_of::<Scope>())
            .supr(AnyType).build());
        let FunctionType = gc.new_object(TypeType, TypeBuilder::new("Function")
            .size(mem::size_of::<Function>())
            .supr(AnyType).build());
        let NilType = gc.new_object(TypeType, TypeBuilder::new("Nil")
            .supr(AnyType).build());

        let ReaderType = gc.new_object(TypeType, TypeBuilder::new("Reader")
            .size(mem::size_of::<Reader>())
            .supr(AnyType).build());

        let ListType = gc.new_object(TypeType, TypeBuilder::new("List")
            .supr(AnyType)
            .size(mem::size_of::<List>())
            .build());

        let SymbolType = gc.new_object(TypeType, TypeBuilder::new("Symbol")
            .supr(AnyType)
            .size(mem::size_of::<String>())
            .build());

        let KeywordType = gc.new_object(TypeType, TypeBuilder::new("Keyword")
            .supr(AnyType)
            .size(mem::size_of::<String>())
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
            .supr(IntegerType).size(mem::size_of::<bool>()).is_bits().build());
        let CharType = gc.new_object(TypeType, TypeBuilder::new("Char")
            .supr(AnyType).size(mem::size_of::<char>()).is_bits().build());

        let Int8Type = gc.new_object(TypeType, TypeBuilder::new("Int8")
            .supr(SignedType).size(mem::size_of::<i8>()).is_bits().build());
        let Int16Type = gc.new_object(TypeType, TypeBuilder::new("Int16")
            .supr(SignedType).size(mem::size_of::<i16>()).is_bits().build());
        let Int32Type = gc.new_object(TypeType, TypeBuilder::new("Int32")
            .supr(SignedType).size(mem::size_of::<i32>()).is_bits().build());
        let Int64Type = gc.new_object(TypeType, TypeBuilder::new("Int64")
            .supr(SignedType).size(mem::size_of::<i64>()).is_bits().build());

        let UInt8Type = gc.new_object(TypeType, TypeBuilder::new("UInt8")
            .supr(UnsignedType).size(mem::size_of::<u8>()).is_bits().build());
        let UInt16Type = gc.new_object(TypeType, TypeBuilder::new("UInt16")
            .supr(UnsignedType).size(mem::size_of::<u16>()).is_bits().build());
        let UInt32Type = gc.new_object(TypeType, TypeBuilder::new("UInt32")
            .supr(UnsignedType).size(mem::size_of::<u32>()).is_bits().build());
        let UInt64Type = gc.new_object(TypeType, TypeBuilder::new("UInt64")
            .supr(UnsignedType).size(mem::size_of::<u64>()).is_bits().build());

        let Float32Type = gc.new_object(TypeType, TypeBuilder::new("Float32")
            .supr(FloatType).size(mem::size_of::<f32>()).is_bits().build());
        let Float64Type = gc.new_object(TypeType, TypeBuilder::new("Float64")
            .supr(FloatType).size(mem::size_of::<f64>()).is_bits().build());

        let namespace_core = gc.new_object(ScopeType, Scope::new(
            Some(gc.new_object(SymbolType, Symbol::new("core.lang".into()))),
            None
        ));

        let mut namespaces = HashMap::new();

        namespaces.insert((**namespace_core.name().unwrap()).clone(), namespace_core);

        Context {
            AnyType: AnyType,
            TypeType: TypeType,

            ScopeType: ScopeType,
            FunctionType: FunctionType,
            NilType: NilType,

            ReaderType: ReaderType,

            ListType: ListType,
            SymbolType: SymbolType,
            KeywordType: KeywordType,

            NumberType: NumberType,
            RealType: RealType,
            FloatType: FloatType,
            IntegerType: IntegerType,
            SignedType: SignedType,
            UnsignedType: UnsignedType,

            BooleanType: BooleanType,
            CharType: CharType,

            Int8Type: Int8Type,
            Int16Type: Int16Type,
            Int32Type: Int32Type,
            Int64Type: Int64Type,

            UInt8Type: UInt8Type,
            UInt16Type: UInt16Type,
            UInt32Type: UInt32Type,
            UInt64Type: UInt64Type,

            Float32Type: Float32Type,
            Float64Type: Float64Type,

            namespaces: namespaces,

            gc: gc,
        }
    }
}
