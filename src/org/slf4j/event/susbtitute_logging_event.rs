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

use std::sync::Arc;

use crate::{org::slf4j::{Marker, SharedLogger, event::Level}, utils::{Argument, Throwable}};

pub struct SubstituteLoggingEvent;

impl SubstituteLoggingEvent {
    pub fn new()->Self{todo!()}
    pub fn set_time_stamp(&mut self, stamp:i64){todo!()}
    pub fn set_level(&mut self, level:Level){todo!()}
    pub fn set_logger(&mut self, logger:SharedLogger){todo!()}
    pub fn set_logger_name(&mut self, name:&String){todo!()}
    pub fn add_marker(&mut self, marker:Box<dyn Marker>){todo!()}
    pub fn set_message(&mut self, msg:String){todo!()}
    pub fn set_thread_name(&mut self, name:String){todo!()}
    pub fn set_args(&mut self, args:Vec<Argument>){todo!()}
    pub fn set_throwable(&mut self, throwable:Option<Throwable>){todo!()}
}