use std::{fmt::{Debug, Display}, hash::Hasher};

// This enum is used to represent an argument according to what has been understood so far
pub enum Argument {
    Value(Box<dyn Debug + Send + Sync>),
    Lazy(Box<dyn Fn() -> Box<dyn Debug + Send + Sync> + Send + Sync>),
}

pub trait ValueTrait: Debug + Display + Send + Sync + 'static {
    fn eq_dyn(&self, other: &dyn ValueTrait) -> bool;
    fn hash_dyn(&self, state: &mut dyn Hasher);

    fn ne_dyn(&self, other: &dyn ValueTrait) -> bool {
        !self.eq_dyn(other)
    }
}

impl ValueTrait for Box<dyn ValueTrait> {
    fn eq_dyn(&self, other: &dyn ValueTrait) -> bool {
        (**self).eq_dyn(other)
    }
    fn hash_dyn(&self, state: &mut dyn Hasher) {
        (**self).hash_dyn(state)
    }
}