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

use crate::{
    org::slf4j::{
        Marker, event::{
            KeyValuePair, 
            Level
        }
    }, utils::{
        Argument,
        Throwable,
        StackTraceElement
    }
};

pub trait LoggingEvent{
    fn get_level(&self)->Level;
    fn get_logger_name(&self)->&String;
    fn get_message(&self)->&String;
    fn get_arguments(&self)->&[Argument];
    fn get_markers(&self)->&[Box<dyn Marker>];
    fn get_key_value_pair(&self)->&[KeyValuePair];
    fn get_throwable(&self)->Option<Throwable>;
    fn get_time_stamp(&self)->i64;
    fn get_thread_name(&self)->&String;
    fn get_caller_boundary(&self)->&String;
    fn get_caller_data(&self) -> &[StackTraceElement];
}