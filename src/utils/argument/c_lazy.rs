use super::{
    Value, 
    CValue
};

// Ojo: NO lleva `Fn() -> Value` como supertrait, porque `dyn CLazy`
// no implementa `Fn()` (solo `dyn Fn()` es especial). En vez de eso
// exponemos `call(&self) -> Value`.

pub trait CLazy: Fn() -> Value + Send + Sync + 'static {
    fn call(&self) -> Value;
    fn clone_box(&self) -> Box<dyn CLazy>;
}

impl<F> CLazy for F
where
    F: Fn() -> Value + Send + Sync + Clone + 'static,
{
    fn call(&self) -> Value {
        self()
    }
    fn clone_box(&self) -> Box<dyn CLazy> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CLazy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}