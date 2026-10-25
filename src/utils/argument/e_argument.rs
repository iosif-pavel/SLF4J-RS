use std::sync::Arc;

use crate::utils::argument::{Lazy, Value};

pub enum EArgument {
    Value(Value),
    Lazy(Lazy),
    Void,
}

impl EArgument {
    pub fn clone(&self)->Self{
        match self {
            EArgument::Value(v) => EArgument::Value(v.clone()),
            EArgument::Lazy(l) => EArgument::Lazy(l.clone()),
            EArgument::Void => EArgument::Void
        }
    }

    pub fn call(&self)->Value{
        match self.clone() {
            EArgument::Value(v) => v.clone(),
            EArgument::Lazy(l) => l(),
            EArgument::Void => {
                Arc::new("VOID")
            }
        }
    }
}