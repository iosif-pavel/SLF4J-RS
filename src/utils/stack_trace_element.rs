// ────────────────────────────────────────────────────────────────────────────
// StackTraceElement
// ────────────────────────────────────────────────────────────────────────────

use std::fmt;

/// Equivalente a `java.lang.StackTraceElement`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StackTraceElement {
    declaring_class: String,
    method_name: String,
    file_name: Option<String>,
    line_number: Option<u32>,
    is_native_method: bool,
}

impl StackTraceElement {
    pub fn new(
        declaring_class: impl Into<String>,
        method_name: impl Into<String>,
        file_name: Option<impl Into<String>>,
        line_number: Option<u32>,
    ) -> Self {
        Self::new_native(declaring_class, method_name, file_name.map(Into::into), line_number, false)
    }

    pub fn new_native(
        declaring_class: impl Into<String>,
        method_name: impl Into<String>,
        file_name: Option<String>,
        line_number: Option<u32>,
        native: bool
    )-> Self{
        Self {
            declaring_class: declaring_class.into(),
            method_name: method_name.into(),
            file_name: file_name,
            line_number,
            is_native_method: native,
        }
    }

    pub fn declaring_class(&self) -> &str { &self.declaring_class }
    pub fn method_name(&self) -> &str { &self.method_name }
    pub fn file_name(&self) -> Option<&str> { self.file_name.as_deref() }
    pub fn line_number(&self) -> Option<u32> { self.line_number }
    pub fn is_native_method(&self) -> bool { self.is_native_method }

    pub fn with_native_method(mut self, yes: bool) -> Self {
        self.is_native_method = yes;
        self
    }
}

impl fmt::Display for StackTraceElement {
    /// Idéntico a `java.lang.StackTraceElement#toString`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}(", self.declaring_class, self.method_name)?;
        if self.is_native_method {
            f.write_str("Native Method")
        } else if let (Some(file), Some(line)) = (&self.file_name, self.line_number) {
            write!(f, "{file}:{line}")
        } else if let Some(file) = &self.file_name {
            f.write_str(file)
        } else {
            f.write_str("Unknown Source")
        }?;
        f.write_str(")")
    }
}