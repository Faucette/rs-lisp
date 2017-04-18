use collections::string::String;
use collections::vec::Vec;

use core::{fmt, mem, ptr};
use core::hash::Hasher;

use ::{Context, Hash, Ptr};

use super::function::Function;
use super::object::Object;
use super::value::Value;
use super::list::List;
use super::symbol::Symbol;
use super::keyword::Keyword;
use super::scope::Scope;
use super::_struct::Struct;
use super::interface:;Interface;


pub struct Type {
    pub(crate) name: String,

    pub(crate) fields: Option<Vec<Ptr<Object<Keyword>>>>,
    //pub(crate) types: Option<Vec<Ptr<Object<Type>>>>,

    pub(crate) constructor: Option<Ptr<Object<Function>>>,

    pub(crate) implements: Vec<Ptr<Object<Interface>>>,

    //pub(crate) size: usize,

    pub(crate) is_bits: bool,
}

unsafe impl Send for Type {}
unsafe impl Sync for Type {}

impl Type {

    #[inline(always)]
    pub fn is_bits(&self) -> bool {
        self.is_bits
    }

    #[inline]
    pub fn new(context: &Context, name_value: Ptr<Value>, fields_value: Ptr<Value>) -> Ptr<Object<Type>> {
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

        let typ_value = {
            if fields_value.typ() == context.ListType {
                let list = fields_value.downcast::<Object<List>>().unwrap();

                TypeBuilder::new(name.as_str())
                    .size(mem::size_of::<Struct>())
                    .fields(list.iter().map(|v| Keyword::to_keyword(context, v)).collect())
                    .build()

            } else if fields_value.typ() == context.UIntType {
                let size = fields_value.downcast::<Object<usize>>().unwrap();

                TypeBuilder::new(name.as_str())
                    .size(*size.value()).is_bits().build()

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

        Type::new(context, name, fields).as_value()
    }
}

impl Hash for Type {

    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);

        match self.fields {
            Some(ref fields) => {
                for field in fields.iter() {
                    Hash::hash(field.value(), state);
                }
            },
            None => (),
        }
        match self.constructor {
            Some(ref constructor) => Hash::hash(&**constructor, state),
            None => (),
        }
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
                write!(f, "(type {} (", self.name)?;
                let mut it = fields.iter();
                while let Some(key) = it.next() {
                    let (size, _) = it.size_hint();

                    if size > 0 {
                        write!(f, "{} ", key)?;
                    } else {
                        write!(f, "{}", key)?;
                    }
                }
                write!(f, "))")
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

    fields: Option<Vec<Ptr<Object<Keyword>>>>,
    types: Option<Vec<Ptr<Object<Type>>>>,

    constructor: Option<Ptr<Object<Function>>>,

    size: usize,

    is_bits: bool,
}

impl TypeBuilder {
    #[inline(always)]
    pub fn new(name: &str) -> Self {
        TypeBuilder {
            name: name.into(),

            fields: None,
            types: None,

            constructor: None,

            size: 0usize,

            is_bits: false,
        }
    }

    #[inline]
    pub fn fields(mut self, fields: Vec<Ptr<Object<Keyword>>>) -> Self {
        self.fields = Some(fields);
        self
    }
    #[inline]
    pub fn types(mut self, types: Vec<Ptr<Object<Type>>>) -> Self {
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
    pub fn is_bits(mut self) -> Self {
        self.is_bits = true;
        self
    }
    #[inline]
    pub fn build(self) -> Type {
        Type {
            name: self.name,

            fields: self.fields,
            //types: self.types,

            constructor: self.constructor,

            //size: self.size,

            is_bits: self.is_bits,
        }
    }
}
