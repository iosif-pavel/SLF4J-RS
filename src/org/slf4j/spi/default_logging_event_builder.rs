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

use crate::org::slf4j::{
    event::Level, 
    SharedLogger, 
    spi::{
        CallerBoundaryAware,
        LoggingEventBuilder
    }
};

pub struct DefaultLoggingEventBuilder;

impl LoggingEventBuilder for DefaultLoggingEventBuilder {
    fn set_cause(&mut self, cause:crate::utils::Throwable) {
        todo!()
    }

    fn add_marker(&mut self, marker: &dyn crate::org::slf4j::Marker) {
        todo!()
    }

    fn add_argument(&mut self, arg:crate::utils::Argument) {
        todo!()
    }

    fn add_key_value(&mut self, key:String, value:Box<dyn crate::utils::Value>) {
        todo!()
    }

    fn with_caller_data_depth(&mut self, depth:u8) {
        todo!()
    }

    fn set_message(&mut self, msg:String) {
        todo!()
    }

    fn set_message_from_supplier(&mut self, msg:Box<dyn Fn()->String>) {
        todo!()
    }

    fn save_log(&mut self) {
        todo!()
    }

    fn log_with_msg(&mut self, msg:String) {
        todo!()
    }

    fn log_with_msg_and_arg(&mut self, msg:String, arg:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn log_with_msg_from_supplier(&mut self, msg:Box<dyn Fn()->String>) {
        todo!()
    }
}
impl CallerBoundaryAware for DefaultLoggingEventBuilder {
    fn set_caller_boundary(&mut self, fqcn:String) {
        todo!()
    }
}
impl DefaultLoggingEventBuilder {
    pub fn new(logger:SharedLogger, level: Level) -> Self{
        todo!()
    }
}