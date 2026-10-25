use std::{
    fmt::{
        Debug, 
        Display
    }, 
    sync::Arc
};

pub trait CValue: Display + Debug + Send + Sync + 'static {
    fn clone_box(&self)->Arc<dyn CValue>;
}

// Blanket impl: cualquier T: Debug + Send + Sync + Clone + 'static
// automáticamente es CValue. Si necesitas un tipo NO clonable como CValue,
// tendrás que quitar este impl y hacer los `clone_box` a mano.
impl<T> CValue for T
where
    T: Display + Debug + Send + Sync + Clone + 'static,
{
    fn clone_box(&self) -> Arc<dyn CValue> {
        Arc::new(self.clone())
    }
}
