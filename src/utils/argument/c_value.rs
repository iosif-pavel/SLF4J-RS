use std::fmt::{
    Debug, 
    Display
};

pub trait CValue: Display + Debug + Send + Sync + 'static {
    fn clone_box(&self)->Box<dyn CValue>;
}

// Blanket impl: cualquier T: Debug + Send + Sync + Clone + 'static
// automáticamente es CValue. Si necesitas un tipo NO clonable como CValue,
// tendrás que quitar este impl y hacer los `clone_box` a mano.
impl<T> CValue for T
where
    T: Display + Debug + Send + Sync + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn CValue> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CValue> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}