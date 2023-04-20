use std::{
    ops::{Deref, Range},
    rc::Rc,
};

pub type Domain = Range<f64>;

pub trait FuncType: Fn(f64) -> f64 {}

#[derive(Debug, Clone, PartialEq)]
pub struct Function<Func: Fn(f64) -> f64> {
    func: Rc<Func>,
    domain: Domain,
}

impl<Func: Fn(f64) -> f64> Function<Func> {
    pub fn new(func: Rc<Func>, domain: Domain) -> Self {
        Self {
            func: Rc::clone(&func),
            domain,
        }
    }

    pub fn domain(&self) -> &Domain {
        &self.domain
    }

    pub fn domain_tuple(&self) -> (f64, f64) {
        (self.domain.start, self.domain.end)
    }

    pub fn function(&self) -> Rc<Func> {
        Rc::clone(&self.func)
    }
}

impl<Func: Fn(f64) -> f64> Deref for Function<Func> {
    type Target = Func;

    fn deref(&self) -> &Self::Target {
        &self.func
    }
}
