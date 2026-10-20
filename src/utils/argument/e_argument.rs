use crate::utils::argument::{Lazy, Value};

pub enum EArgument {
    Value(Value),
    Lazy(Lazy)
}

impl EArgument {
    pub fn clone(&self)->Self{
        match self {
            EArgument::Value(v) => EArgument::Value(v.clone_box()),
            EArgument::Lazy(l) => EArgument::Lazy(l.clone_box())
        }
    }

    pub fn call(&self)->Value{
        match self.clone() {
            EArgument::Value(v) => v,
            EArgument::Lazy(l) => l.call()
        }
    }
}