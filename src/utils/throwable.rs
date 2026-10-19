use std::{
    fmt, 
    io
};
use crate::utils::{
    StackTraceElement, 
    convert_backtrace
};

/// Equivalente a `java.lang.Throwable` — representación explícita de un
/// error con mensaje, causa, stack trace y suprímidos.
#[derive(Clone)]
pub struct Throwable {
    kind: String,
    message: Option<String>,
    cause: Option<Box<Throwable>>,
    stack_trace: Vec<StackTraceElement>,
    suppressed: Vec<Throwable>,
}

impl Throwable {
    /// Construye un Throwable vacío (sin mensaje, sin stack).
    pub fn new() -> Self {
        Self {
            kind: "Throwable".to_string(),
            message: None,
            cause: None,
            stack_trace: Vec::new(),
            suppressed: Vec::new(),
        }
    }

    /// Solo con mensaje, sin stack (para rellenar luego).
    pub fn with_message(message: impl Into<String>) -> Self {
        Self { message: Some(message.into()), ..Self::new() }
    }

    /// Con mensaje y "clase" (ej. `"java.lang.IllegalStateException"`).
    pub fn with_kind_and_message(kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self { kind: kind.into(), message: Some(message.into()), ..Self::new() }
    }

    /// Captura el stack actual **y** guarda el mensaje.
    pub fn capture(message: impl Into<String>) -> Self {
        let mut t = Self::with_message(message);
        t.fill_in_stack_trace();
        t
    }

    /// Captura stack + causa ya construida.
    pub fn capture_caused_by(message: impl Into<String>, cause: Throwable) -> Self {
        let mut t = Self {
            cause: Some(Box::new(cause)),
            ..Self::with_message(message)
        };
        t.fill_in_stack_trace();
        t
    }

    // ── construcción "mutable" estilo Java ──────────────────────────────

    pub fn set_cause(&mut self, cause: Throwable) {
        self.cause = Some(Box::new(cause));
    }

    pub fn set_stack_trace(&mut self, trace: Vec<StackTraceElement>) {
        self.stack_trace = trace;
    }

    pub fn clear_stack_trace(&mut self) {
        self.stack_trace.clear();
    }

    pub fn add_suppressed(&mut self, t: Throwable) {
        self.suppressed.push(t);
    }

    /// Captura el stack en el punto actual (equivalente a `fillInStackTrace`).
    pub fn fill_in_stack_trace(&mut self) {
        let bt = backtrace::Backtrace::new();
        // skip: `Backtrace::new` + internals + este método
        self.stack_trace = convert_backtrace(&bt, 2);
    }

    // ── accesores ───────────────────────────────────────────────────────

    pub fn kind(&self) -> &str { &self.kind }
    pub fn message(&self) -> Option<&str> { self.message.as_deref() }
    pub fn cause(&self) -> Option<&Throwable> { self.cause.as_deref() }
    pub fn stack_trace(&self) -> &[StackTraceElement] { &self.stack_trace }
    pub fn suppressed(&self) -> &[Throwable] { &self.suppressed }

    // ── impresión ───────────────────────────────────────────────────────

    /// `printStackTrace()` de Java, escribiendo en el writer indicado.
    pub fn print_stack_trace_to<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "{self}")?;
        for frame in &self.stack_trace {
            writeln!(w, "\tat {frame}")?;
        }
        for sup in &self.suppressed {
            write!(w, "\tSuppressed: ")?;
            sup.print_stack_trace_to(w)?;
        }
        if let Some(cause) = &self.cause {
            write!(w, "Caused by: ")?;
            cause.print_stack_trace_to(w)?;
        }
        Ok(())
    }

    /// Atajo a `stderr`.
    pub fn print_stack_trace(&self) {
        let _ = self.print_stack_trace_to(&mut io::stderr());
    }

    /// Devuelve el stack formateado como `String` — útil para el logger.
    pub fn to_stack_trace_string(&self) -> String {
        let mut buf = Vec::new();
        let _ = self.print_stack_trace_to(&mut buf);
        String::from_utf8_lossy(&buf).into_owned()
    }

    /// Convierte un error de Rust (y su cadena de `source()`) a `Throwable`.
    ///
    /// **Nota**: el stack capturado es el del *sitio de conversión*, no
    /// donde se originó el error. Si te importa esa distinción, convierte
    /// lo antes posible.
    pub fn from_error(err: &(dyn std::error::Error + 'static)) -> Self {
        fn build(err: &(dyn std::error::Error + 'static)) -> Throwable {
            let mut t = Throwable::with_kind_and_message("RustError", err.to_string());
            t.fill_in_stack_trace();
            if let Some(src) = err.source() {
                t.cause = Some(Box::new(build(src)));
            }
            t
        }
        build(err)
    }
}

impl Default for Throwable {
    fn default() -> Self { Self::new() }
}

impl fmt::Display for Throwable {
    /// `Throwable#toString()` de Java: `"Kind: message"`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.kind)?;
        if let Some(msg) = &self.message {
            write!(f, ": {msg}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for Throwable {
    /// Compacto — usa `print_stack_trace()` si quieres la traza completa.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::error::Error for Throwable {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause
            .as_ref()
            .map(|c| c.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// Conversiones ergonómicas desde los errores de Rust.
impl From<Box<dyn std::error::Error + 'static>> for Throwable {
    fn from(err: Box<dyn std::error::Error + 'static>) -> Self {
        Throwable::from_error(err.as_ref())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for Throwable {
    fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        Throwable::from_error(err.as_ref())
    }
}