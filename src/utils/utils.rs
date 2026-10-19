use std::path::Path;

use crate::utils::StackTraceElement;

pub fn convert_backtrace(bt: &backtrace::Backtrace, skip: usize) -> Vec<StackTraceElement> {
    bt.frames()
        .iter()
        .skip(skip)
        .filter_map(frame_to_element)
        .collect()
}

pub fn frame_to_element(frame: &backtrace::BacktraceFrame) -> Option<StackTraceElement>{
    if let Some(symbol) = frame.symbols().iter().find(|s| s.name().is_some()) {
        if let Some(name) = symbol.name().and_then(|n| n.as_str()){
            let (class, method) = split_rust_symbol(name);
            let file_name = symbol
                .filename()
                .and_then(Path::file_name)
                .and_then(|s| s.to_str())
                .map(str::to_string);

            return Some(
                StackTraceElement::new(
                    class, 
                    method, 
                    file_name, 
                    symbol.lineno()
                )
            );
        }
    }

    // Frame sin simbolo: Se reporta como método nativo coon la IP
    Some(
        StackTraceElement::new_native(
            "<native>", 
            format!("0x{:x}", frame.ip() as usize),
            None, 
            None, 
            true
        )
    )
}

/// A partir de `crate::mod::Type::method::h1234abcd`, devuelve
/// `("crate::mod::Type", "method")`. Heurística simple pero suficiente.
pub fn split_rust_symbol(sym: &str) -> (String, String) {
    // Quitar el hash de mangling legado "::h<hex>" si existe.
    let s = match sym.rfind("::h") {
        Some(pos)
            if sym[pos + 3..].chars().all(|c| c.is_ascii_hexdigit())
                && sym[pos + 3..].len() >= 8 =>
        {
            &sym[..pos]
        }
        _ => sym,
    };
    match s.rfind("::") {
        Some(pos) => (s[..pos].to_string(), s[pos + 2..].to_string()),
        None => (String::new(), s.to_string()),
    }
}
