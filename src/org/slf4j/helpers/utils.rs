// SPDX-License-Identifier: MIT
/*
 * Copyright (C) 2004-2026, QOS.ch (Switzerland)
 * All rights reserved.
 *
 *  Permission is hereby granted, free  of charge, to any person obtaining
 *  a  copy  of this  software  and  associated  documentation files  (the
 *  "Software"), to  deal in  the Software without  restriction, including
 *  without limitation  the rights to  use, copy, modify,  merge, publish,
 *  distribute,  sublicense, and/or sell  copies of  the Software,  and to
 *  permit persons to whom the Software  is furnished to do so, subject to
 *  the following conditions:
 *
 *  The  above  copyright  notice  and  this permission  notice  shall  be
 *  included in all copies or substantial portions of the Software.
 *
 *  THE  SOFTWARE IS  PROVIDED  "AS  IS", WITHOUT  WARRANTY  OF ANY  KIND,
 *  EXPRESS OR  IMPLIED, INCLUDING  BUT NOT LIMITED  TO THE  WARRANTIES OF
 *  MERCHANTABILITY,    FITNESS    FOR    A   PARTICULAR    PURPOSE    AND
 *  NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
 *  LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
 *  OF CONTRACT, TORT OR OTHERWISE,  ARISING FROM, OUT OF OR IN CONNECTION
 *  WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

//! Utilidades internas del port SLF4J → Rust.

use std::{
    collections::HashMap,
    env,
    sync::{
        OnceLock,
        RwLock
    }
};

// ────────────────────────────────────────────────────────────────────────
// "System properties"
// ────────────────────────────────────────────────────────────────────────
//
// En Java `System.getProperty` puede leer de:
//   - argumentos `-Dkey=value` de la JVM
//   - `System.setProperty` en runtime
//   - `System.getProperties()` (que a su vez arranca con env vars)
//
// El equivalente más cercano en Rust es combinar:
//   1. Un registro mutable en proceso (para `set_system_property`).
//   2. `env::var` como fallback (equivale a las env vars / -D vía env).
//
// Nota: `std::env::set_var` está marcado como `unsafe` en Rust 2024
// edition por seguridad entre hilos, así que NO lo usamos; el registro
// con `RwLock` es la forma segura.

fn properties() -> &'static RwLock<HashMap<String, String>> {
    static PROPS: OnceLock<RwLock<HashMap<String, String>>> = OnceLock::new();
    PROPS.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Análogo de `System.setProperty`.
pub fn set_system_property(key: impl Into<String>, value: impl Into<String>) {
    properties()
        .write()
        .expect("properties lock poisoned")
        .insert(key.into(), value.into());
}

/// Análogo de `System.getProperty`.
fn get_system_property(key: &str) -> Option<String> {
    if let Some(v) = properties()
        .read()
        .expect("properties lock poisoned")
        .get(key)
        .cloned()
    {
        return Some(v);
    }
    // Fallback a variables de entorno.
    let env_var = env::var(key).ok();
    if let Some(var) = &env_var{
        self::set_system_property(key, var);
    }

    env_var
}

/// Equivalente de `safeGetSystemProperty`.
///
/// Diferencias con Java:
/// - `key: &str` no puede ser `null`, así que eliminamos el
///   `IllegalArgumentException("null input")` — lo cubre el tipo.
/// - No hay `SecurityException`; `env::var` ya devuelve `Result`.
pub fn safe_get_system_property(key: &str) -> Option<String> {
    get_system_property(key)
}

/// Equivalente de `safeGetBooleanSystemProperty`.
pub fn safe_get_boolean_system_property(key: &str) -> bool {
    safe_get_system_property(key)
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

// ────────────────────────────────────────────────────────────────────────
// report(...) — deprecado también en el original
// ────────────────────────────────────────────────────────────────────────

/// Deprecado. Usa `Reporter::error` en su lugar.
#[deprecated(note = "replaced by Reporter::error")]
pub fn report_with_error(msg: &str, t: &(dyn std::error::Error + 'static)) {
    eprintln!("{msg}");
    eprintln!("Reported exception:");
    eprintln!("{t:?}");
}

/// Deprecado. Usa `Reporter::info/warn/error` en su lugar.
#[deprecated(note = "replaced by Reporter::info/warn/error")]
pub fn report(msg: &str) {
    eprintln!("SLF4J: {msg}");
}

// ────────────────────────────────────────────────────────────────────────
// getCallingClass — NO se porta como método runtime
// ────────────────────────────────────────────────────────────────────────
//
// Reemplazo idiomático en el sitio de llamada:
//
//     let logger = LoggerFactory::get_logger(module_path!());
//
// Si un día necesitas el stack real, añade la crate `backtrace` y filtra
// frames — pero considera que es frágil y no portable.