use std::{
    fmt::Debug, 
    sync::Arc
};

type Lazy = Box<dyn CLazy>;
type Value = Box<dyn CValue>;

// ── CValue ─────────────────────────────────────────────────────────────
pub trait CValue: Debug + Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn CValue>;
}

// Blanket impl: cualquier T: Debug + Send + Sync + Clone + 'static
// automáticamente es CValue. Si necesitas un tipo NO clonable como CValue,
// tendrás que quitar este impl y hacer los `clone_box` a mano.
impl<T> CValue for T
where
    T: Debug + Send + Sync + Clone + 'static,
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

// ── CLazy ──────────────────────────────────────────────────────────────
// Ojo: NO lleva `Fn() -> Value` como supertrait, porque `dyn CLazy`
// no implementa `Fn()` (solo `dyn Fn()` es especial). En vez de eso
// exponemos `call(&self) -> Value`.
pub trait CLazy: Send + Sync + 'static {
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

// ── EArgument ──────────────────────────────────────────────────────────
enum EArgument {
    Value(Value),
    Lazy(Lazy),
}

impl Clone for EArgument {
    fn clone(&self) -> Self {
        match self {
            EArgument::Value(v) => EArgument::Value(v.clone()),
            EArgument::Lazy(l) => EArgument::Lazy(l.clone()),
        }
    }
}

// ── SArgument (antes SEArgument) ───────────────────────────────────────
struct SArgument {
    arg: EArgument,
}

impl Clone for SArgument {
    fn clone(&self) -> Self {
        Self { arg: self.arg.clone() }
    }
}

impl SArgument {
    pub fn new_value(value: Value) -> Self {
        Self { arg: EArgument::Value(value) }
    }

    pub fn new_lazy(lazy: Lazy) -> Self {
        Self { arg: EArgument::Lazy(lazy) }
    }

    // Ahora por &self — no puede consumir porque vive dentro de un Arc.
    pub fn get(&self) -> Value {
        match &self.arg {
            EArgument::Value(v) => v.clone(),
            EArgument::Lazy(l) => l.call(),
        }
    }
}

// ── Argument ───────────────────────────────────────────────────────────
#[derive(Clone)]
pub struct Argument {
    arg: Arc<SArgument>,
}

impl Argument {
    pub fn new_value(value: Value) -> Self {
        Self { arg: Arc::new(SArgument::new_value(value)) }
    }

    pub fn new_lazy(lazy: Lazy) -> Self {
        Self { arg: Arc::new(SArgument::new_lazy(lazy)) }
    }

    pub fn get(&self) -> Value {
        self.arg.get() // Deref: &Arc<SArgument> → &SArgument
    }
}