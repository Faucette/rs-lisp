use super::super::gc::GcObject;
use super::scope::Scope;
use super::object::init_typs;


pub struct Context {
    pub scope: GcObject<Scope>,
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}

impl Context {

    #[inline(always)]
    pub fn new() -> Self {
        let mut scope = GcObject::new(Scope::new(None));

        init_typs(&mut *scope);

        Context {
            scope: scope,
        }
    }
}
