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

use crate::{
    org::slf4j::{
        event::Level, 
        spi::{
            DefaultLoggingEventBuilder, 
            LoggingEventBuilder, 
            NOPLoggingEventBuilder
        },
        Marker
    }, 
    utils::{
        Argument,
        Throwable
    }
};

pub type SharedLogger = Arc<Box<dyn Logger + Send + Sync + 'static>>;

pub const ROOT_LOGGER_NAME:&'static str = "ROOT";

pub trait Logger {
    fn get_name(&self)->&String;
    fn make_logging_event_builder(&self, logger:SharedLogger, level:Level)->Arc<dyn LoggingEventBuilder>{
        let dleb = DefaultLoggingEventBuilder::new(logger, level);
        Arc::new(dleb)
    }
    
    fn at_level(&self, logger:SharedLogger, level:Level)->Arc<dyn LoggingEventBuilder>{
        if self.is_enabled_for_level(&level){
            self.make_logging_event_builder(logger, level)
        }else{
            NOPLoggingEventBuilder::singleton()
        }
    }
    fn is_enabled_for_level(&self, level:&Level)->bool{
        match level {
            &Level::TRACE => self.is_trace_enabled(),
            &Level::DEBUG => self.is_debug_enabled(),
            &Level::INFO => self.is_info_enabled(),
            &Level::WARN => self.is_warn_enabled(),
            &Level::ERROR => self.is_error_enabled(),
            _ => false,
        }
    }
    
    fn is_trace_enabled(&self)->bool;
    fn at_trace(&self, logger:SharedLogger)->Arc<dyn LoggingEventBuilder>{
        if self.is_trace_enabled() {
            self.make_logging_event_builder(logger, Level::TRACE)
        }else {
            NOPLoggingEventBuilder::singleton()
        }
    }
    fn trace_with_msg(&self, msg:String);
    fn trace_with_arg(&self, format:String, args:Vec<Argument>);
    fn trace_with_msg_and_error(&self, msg:String, t:Throwable);
    fn is_trace_from_marker_enabled(&self, marker:Box<dyn Marker>);
    fn is_trace_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String);
    fn is_trace_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>);
    fn is_trace_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable);

    fn is_debug_enabled(&self)->bool;
    fn at_debug(&self, logger:SharedLogger)->Arc<dyn LoggingEventBuilder>{
        if self.is_debug_enabled() {
            self.make_logging_event_builder(logger, Level::DEBUG)
        }else {
            NOPLoggingEventBuilder::singleton()
        }
    }
    fn debug_with_msg(&self, msg:String);
    fn debug_with_arg(&self, format:String, args:Vec<Argument>);
    fn debug_with_msg_and_error(&self, msg:String, t:Throwable);
    fn is_debug_from_marker_enabled(&self, marker:Box<dyn Marker>);
    fn is_debug_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String);
    fn is_debug_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>);
    fn is_debug_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable);
    
    fn is_info_enabled(&self)->bool;
    fn at_(&self, logger:SharedLogger)->Arc<dyn LoggingEventBuilder>{
        if self.is_info_enabled() {
            self.make_logging_event_builder(logger, Level::INFO)
        }else {
            NOPLoggingEventBuilder::singleton()
        }
    }
    fn info_with_msg(&self, msg:String);
    fn info_with_arg(&self, format:String, args:Vec<Argument>);
    fn info_with_msg_and_error(&self, msg:String, t:Throwable);
    fn is_info_from_marker_enabled(&self, marker:Box<dyn Marker>);
    fn is_info_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String);
    fn is_info_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>);
    fn is_info_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable);
    
    fn is_warn_enabled(&self)->bool;
    fn at_warn(&self, logger:SharedLogger)->Arc<dyn LoggingEventBuilder>{
        if self.is_warn_enabled() {
            self.make_logging_event_builder(logger, Level::WARN)
        }else {
            NOPLoggingEventBuilder::singleton()
        }
    }
    fn warn_with_msg(&self, msg:String);
    fn warn_with_arg(&self, format:String, args:Vec<Argument>);
    fn warn_with_msg_and_error(&self, msg:String, t:Throwable);
    fn is_warn_from_marker_enabled(&self, marker:Box<dyn Marker>);
    fn is_warn_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String);
    fn is_warn_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>);
    fn is_warn_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable);
    
    fn is_error_enabled(&self)->bool;
    fn at_error(&self, logger:SharedLogger)->Arc<dyn LoggingEventBuilder>{
        if self.is_error_enabled() {
            self.make_logging_event_builder(logger, Level::ERROR)
        }else {
            NOPLoggingEventBuilder::singleton()
        }
    }
    fn error_with_msg(&self, msg:String);
    fn error_with_arg(&self, format:String, args:Vec<Argument>);
    fn error_with_msg_and_error(&self, msg:String, t:Throwable);
    fn is_error_from_marker_enabled(&self, marker:Box<dyn Marker>);
    fn is_error_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String);
    fn is_error_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>);
    fn is_error_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable);
}
