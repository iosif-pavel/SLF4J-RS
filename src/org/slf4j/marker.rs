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

/**
 * Markers are named objects used to enrich log statements. Conforming logging
 * system implementations of SLF4J should determine how information conveyed by
 * any markers are used, if at all. Many conforming logging systems ignore marker
 * data entirely.
 *
 * <p>Markers can contain references to nested markers, which in turn may
 * contain references of their own. Note that the fluent API (new in 2.0) allows adding
 * multiple markers to a logging statement. It is often preferable to use
 * multiple markers instead of nested markers.
 * </p>
 *
 * @author Ceki G&uuml;lc&uuml;
 */

pub trait Marker: Send + Sync{
    fn get_name(&self)->String;

    fn add(&self, new:&dyn Marker);

    fn remove(&self, marker:&dyn Marker)->bool;

    #[deprecated]
    fn has_child(&self)->bool;

    fn has_references(&self)->bool;

    fn contains_marker(&self, marker:&dyn Marker)->bool;

    fn contains(&self, marker:String)->bool;
    
    fn visit_mut(&self, f: &mut dyn FnMut(&dyn Marker));

    fn visitor_iterator(&self, f: &mut dyn Fn(&dyn Marker));
}

pub trait MarkerConst {
    const ANY_MARKER:&'static str="*";
    const ANY_NON_NULL_MARKER:&'static str="+";
}