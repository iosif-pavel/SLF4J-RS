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

use std::{cell::OnceCell, sync::Arc};

use crate::org::slf4j::{
    spi::LoggingEventBuilder, 
    event::Level, 
    Logger, 
    Marker
};

const  SINGLETON:OnceCell<Arc<NOPLoggingEventBuilder>> = OnceCell::new();

pub struct NOPLoggingEventBuilder;

impl NOPLoggingEventBuilder {
    fn new()->Self{
        Self
    }
    pub fn singleton()->Arc<NOPLoggingEventBuilder>{
        SINGLETON.get_or_init(||Arc::new(Self::new())).clone()
    }
}

impl LoggingEventBuilder for NOPLoggingEventBuilder {
    fn set_cause(&mut self, _cause:crate::utils::Throwable) {}
    fn add_marker(&mut self, _marker: &dyn crate::org::slf4j::Marker) {}
    fn add_argument(&mut self, _arg:crate::utils::Argument) {}
    fn add_key_value(&mut self, _key:String, _value:Box<dyn crate::utils::Value>) {}
    fn with_caller_data_depth(&mut self, _depth:u8) {}
    fn set_message(&mut self, _msg:String) {}
    fn set_message_from_supplier(&mut self, _msg:Box<dyn Fn()->String>) {}
    fn save_log(&mut self) {}
    fn log_with_msg(&mut self, _msg:String) {}
    fn log_with_msg_and_arg(&mut self, _msg:String, _arg:Vec<crate::utils::Argument>) {}
    fn log_with_msg_from_supplier(&mut self, _msg:Box<dyn Fn()->String>) {}
}