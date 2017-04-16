use collections::string::String;

use core::{fmt, mem, ptr};
use core::hash::Hash;

use hash_map::DefaultHasher;
use vector::Vector;

use ::{Context, Ptr};

use super::function::Function;
use super::object::Object;
use super::value::Value;
use super::list::List;
use super::symbol::Symbol;
use super::keyword::Keyword;
use super::scope::Scope;
use super::_struct::Struct;


pub struct Type {
    pub(crate) name: String,

    pub(crate) supr: Option<Ptr<Object<Type>>>,

    pub(crate) fields: Option<Vector<String>>,
    //pub(crate) types: Option<Vector<Ptr<Object<Type>>>>,

    pub(crate) constructor: Option<Ptr<Object<Function>>>,

    //pub(crate) size: usize,

    pub(crate) is_abstract: bool,
    pub(crate) is_bits: bool,
}

unsafe impl Send for Type {}
unsafe impl Sync for Type {}

impl Type {

    #[inline(always)]
    pub fn is_real(&self) -> bool { !self.is_abstract }

    #[inline(always)]
    pub fn is_abstract(&self) -> bool { self.is_abstract }

    #[inline(always)]
    pub fn is_bits(&self) -> bool { self.is_bits }

    #[inline(always)]
    pub fn instance_of(&self, typ: &Type) -> bool {
        if self == typ {
            true
        } else if let Some(supr) = self.supr {
            supr.instance_of(typ)
        } else {
            false
        }
    }

    #[inline]
    pub fn new(context: &Context, name_value: Ptr<Value>, fields_value: Ptr<Value>, supr_value: Ptr<Value>) -> Ptr<Object<Type>> {
        let name: String = {
            if name_value.typ() == context.KeywordType {
                let keyword = name_value.downcast::<Object<Keyword>>().unwrap();
                (*keyword.value()).clone()
            } else if name_value.typ() == context.SymbolType {
                let symbol = name_value.downcast::<Object<Symbol>>().unwrap();
                (*symbol.value()).clone()
            } else {
                String::new()
            }
        };

        let supr = {
            if supr_value.typ() == context.TypeType {
                supr_value.downcast::<Object<Type>>().unwrap()
            } else {
                context.AnyType
            }
        };

        let typ_value = {
            if fields_value.typ() == context.ListType {
                let list = fields_value.downcast::<Object<List>>().unwrap();

                TypeBuilder::new(name.as_str())
                    .size(mem::size_of::<Struct>())
                    .fields(list.iter().map(|v| Struct::key_to_string(context, &v)).collect())
                    .supr(supr).build()

            } else if fields_value.typ() == context.UIntType {
                let size = fields_value.downcast::<Object<usize>>().unwrap();

                TypeBuilder::new(name.as_str())
                    .supr(supr).size(*size.value()).is_bits().build()

            } else if
                fields_value.typ() == context.KeywordType &&
                **fields_value.downcast::<Object<Keyword>>().unwrap().value() == "abstract"
            {
                TypeBuilder::new(name.as_str())
                    .supr(supr).is_abstract().build()
            } else {
                panic!("invalid type argument {:?}", fields_value)
            }
        };

        let mut typ = context.gc.new_object(context.TypeType, typ_value);
        typ.value.constructor = Some(context.gc.new_object(context.FunctionType, Function::new_constructor(typ)));
        typ
    }

    #[inline]
    pub fn constructor(context: &Context, _scope: Ptr<Object<Scope>>, mut args: Ptr<Object<List>>) -> Ptr<Value> {
        let name = args.first(context);
        args = args.pop(context);

        let fields = args.first(context);
        args = args.pop(context);

        let supr = args.first(context);
        //args = args.pop(context);

        Type::new(context, name, fields, supr).as_value()
    }
}

impl Hash for Type {

    #[inline(always)]
    fn hash(&self, state: &mut DefaultHasher) {
        self.name.hash(state);
        self.fields.hash(state);
        match self.constructor {
            Some(constructor) => Hash::hash(&*constructor, state),
            None => (),
        }
        self.is_abstract.hash(state);
        self.is_bits.hash(state);
    }
}

impl PartialEq for Type {

    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}

impl Eq for Type {}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.fields {
            Some(ref fields) => {
                write!(f, "(type {} [", self.name)?;
                let mut it = fields.iter();
                while let Some(key) = it.next() {
                    let (size, _) = it.size_hint();

                    if size > 0 {
                        write!(f, ":{} ", key)?;
                    } else {
                        write!(f, ":{}", key)?;
                    }
                }
                write!(f, "])")
            },
            None => write!(f, "(type {})", self.name)
        }
    }
}

impl fmt::Debug for Type {

    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

pub struct TypeBuilder {
    name: String,

    supr: Option<Ptr<Object<Type>>>,

    fields: Option<Vector<String>>,
    types: Option<Vector<Ptr<Object<Type>>>>,

    constructor: Option<Ptr<Object<Function>>>,

    size: usize,

    is_abstract: bool,
    is_bits: bool,
}

impl TypeBuilder {
    #[inline(always)]
    pub fn new(name: &str) -> Self {
        TypeBuilder {
            name: name.into(),

            supr: None,

            fields: None,
            types: None,

            constructor: None,

            size: 0usize,

            is_abstract: false,
            is_bits: false,
        }
    }

    #[inline]
    pub fn supr(mut self, supr: Ptr<Object<Type>>) -> Self {
        self.supr = Some(supr);
        self
    }
    #[inline]
    pub fn fields(mut self, fields: Vector<String>) -> Self {
        self.fields = Some(fields);
        self
    }
    #[inline]
    pub fn types(mut self, types: Vector<Ptr<Object<Type>>>) -> Self {
        self.types = Some(types);
        self
    }
    #[inline]
    pub fn size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
    #[inline]
    pub fn constructor(mut self, constructor: Ptr<Object<Function>>) -> Self {
        self.constructor = Some(constructor);
        self
    }
    #[inline]
    pub fn is_abstract(mut self) -> Self {
        self.is_abstract = true;
        self
    }
    #[inline]
    pub fn is_bits(mut self) -> Self {
        self.is_bits = true;
        self
    }
    #[inline]
    pub fn build(self) -> Type {
        Type {
            name: self.name,

            supr: self.supr,

            fields: self.fields,
            //types: self.types,

            constructor: self.constructor,

            //size: self.size,

            is_abstract: self.is_abstract,
            is_bits: self.is_bits,
        }
    }
}
