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

use std::hash::Hasher;

pub trait MarkerConstants {
    // Representa a todos los marcadores no nulos
    const ANY_NON_NULL_MARKER:&'static str = "+";

    // Representa a todos los marcadores, incluye a los nulos.
    const ANY_MARKER:&'static str = "*";
}

pub trait Marker {
    // Retorna el nombre del marcador
    fn get_name(&self)->String;

    // Añade una referencia a otro marcador
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn add(&self, reference:dyn Marker);

    // Renueve una referencia
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn remove(&self, reference:dyn Marker)->bool;

    // Verdadero si tiene referencias hijas
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn has_children(&self)->bool;

    // Verdadero si tiene alguna referencia
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn has_references(&self)->bool;
    
    // Version idiomatica de iterator(), pero modifica las referencias internas
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn visit_mut(&self, f: &mut dyn FnMut(&dyn Marker));

    // Version idiomatica de iterator() mas fiel, no modifica los datos internos,
    // pero si permite modificar un objeto externo
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn visitor_iterator(&self, f: &mut dyn Fn(&dyn Marker));

    // Pregunta si contiene un marcador determinado
    #[deprecated(note="En una futura version los marcadores seran inmutables")]
    fn contains(&self, other:dyn Marker)->bool;

    fn eq(&self, other:dyn Marker)->bool;

    fn hash(&self, hasher:&mut dyn Hasher);
}