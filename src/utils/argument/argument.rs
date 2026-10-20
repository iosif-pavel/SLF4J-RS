use std::sync::Arc;

use crate::utils::argument::{Lazy, Value, e_argument::EArgument};

#[derive(Clone)]
pub struct Argument{
    arg:Arc<EArgument>
}

impl Argument {
    pub fn new_value(value:Value)->Self{
        Self{
            arg: Arc::new(
                EArgument::Value(
                    value
                )
            )
        }
    }
    pub fn new_lazy(lazy:Lazy)->Self{
        Self {
            arg: Arc::new(
                EArgument::Lazy(
                    lazy
                )
            )
        }
    }

    pub fn get(&self)->Value{
        self.arg.call()
    }

    pub fn clone(&self)->Argument{
        let arg = self.arg.clone();
        Self{
            arg
        }
    }
}