use std::sync::{Arc, RwLock};
use crate::{org::slf4j::{Logger, Marker, event::event_recording_logger::InternEventRecordingLogger, helpers::{AbstractLogger, LegacyAbstractLogger}}, utils::{Argument, Throwable}};

const RECORD_ALL_EVENTS:bool = true;

#[derive(Clone)]
pub struct EventRecordingLogger{
    intern:Arc<RwLock<InternEventRecordingLogger>>
}

impl LegacyAbstractLogger for EventRecordingLogger {/* All are default methods */}

impl AbstractLogger for EventRecordingLogger {
    fn get_full_qualified_caller_name(&self)-> String {
        if let Ok(reader) = self.intern.read(){
            let v= reader.get_fully_qualified_caller_name();
            v.clone()
        }else {
            String::new()
        }
    }
    fn handle_normalized_logging_call(
        &self, 
        level:Option<crate::org::slf4j::event::Level>, 
        marker:Option<Box<dyn crate::org::slf4j::Marker>>, 
        msg:Option<String>, 
        args:Option<Vec<crate::utils::Argument>>, 
        t:Option<Throwable>
    ) {
        if let Ok(mut writter) = self.intern.write(){
            let args = match args {
                Some(args) => args,
                None => Vec::new()
            };
            writter.handleNormalizedLoggingCall(level, marker, msg, args, t);
        }
    }
}

impl Logger for EventRecordingLogger {
    fn get_name(&self)->String {
        if let Ok(reader) = self.intern.read(){
            reader.get_name().clone()
        }else {
            String::new()
        }
    }
    fn clone_at(&self)->crate::org::slf4j::SharedLogger {
        Arc::new(self.clone())
    }
    fn is_trace_enabled(&self)->bool{
        RECORD_ALL_EVENTS
    }
    fn trace_with_msg(&self, msg:String) {
        AbstractLogger::trace_with_msg(self, msg);
    }
    fn trace_with_arg(&self, format:String, args:Vec<Argument>) {
        AbstractLogger::trace_with_arg(self, format, args);
    }
    fn trace_with_msg_and_error(&self, msg:String, t:Throwable) {
        AbstractLogger::trace_with_msg_and_error(self, msg, t);
    }
    fn is_trace_from_marker_enabled(&self, marker:&Box<dyn Marker>) -> bool {
        LegacyAbstractLogger::is_trace_from_marker_enabled(self, marker)
    }
    fn trace_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        AbstractLogger::trace_from_marker_with_msg(self, marker, msg);
    }
    fn trace_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        AbstractLogger::trace_from_marker_with_arg(self, marker, format, args);
    }
    fn trace_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        AbstractLogger::trace_from_marker_with_msg_and_error(self, marker, msg, t);
    }
    fn is_debug_enabled(&self)->bool{
        RECORD_ALL_EVENTS
    }
    fn debug_with_msg(&self, msg:String) {
        AbstractLogger::debug_with_msg(self, msg);
    }
    fn debug_with_arg(&self, format:String, args:Vec<Argument>) {
        AbstractLogger::debug_with_arg(self, format, args);
    }
    fn debug_with_msg_and_error(&self, msg:String, t:Throwable) {
        AbstractLogger::debug_with_msg_and_error(self, msg, t);
    }
    fn is_debug_from_marker_enabled(&self, marker:&Box<dyn Marker>) -> bool {
        LegacyAbstractLogger::is_debug_from_marker_enabled(self, marker)
    }
    fn debug_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        AbstractLogger::debug_from_marker_with_msg(self, marker, msg);
    }
    fn debug_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        AbstractLogger::debug_from_marker_with_arg(self, marker, format, args);
    }
    fn debug_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        AbstractLogger::debug_from_marker_with_msg_and_error(self, marker, msg, t);
    }
    fn is_info_enabled(&self)->bool{
        RECORD_ALL_EVENTS
    }
    fn info_with_msg(&self, msg:String) {
        AbstractLogger::info_with_msg(self, msg);
    }
    fn info_with_arg(&self, format:String, args:Vec<Argument>) {
        AbstractLogger::info_with_arg(self, format, args);
    }
    fn info_with_msg_and_error(&self, msg:String, t:Throwable) {
        AbstractLogger::info_with_msg_and_error(self, msg, t);
    }
    fn is_info_from_marker_enabled(&self, marker:&Box<dyn Marker>) -> bool {
        LegacyAbstractLogger::is_info_from_marker_enabled(self, marker)
    }
    fn info_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        AbstractLogger::info_from_marker_with_msg(self, marker, msg);
    }
    fn info_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        AbstractLogger::info_from_marker_with_arg(self, marker, format, args);
    }
    fn info_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        AbstractLogger::info_from_marker_with_msg_and_error(self, marker, msg, t);
    }
    fn is_warn_enabled(&self)->bool{
        RECORD_ALL_EVENTS
    }
    fn warn_with_msg(&self, msg:String) {
        AbstractLogger::warn_with_msg(self, msg);
    }
    fn warn_with_arg(&self, format:String, args:Vec<Argument>) {
        AbstractLogger::warn_with_arg(self, format, args);
    }
    fn warn_with_msg_and_error(&self, msg:String, t:Throwable) {
        AbstractLogger::warn_with_msg_and_error(self, msg, t);
    }
    fn is_warn_from_marker_enabled(&self, marker:&Box<dyn Marker>) -> bool {
        LegacyAbstractLogger::is_warn_from_marker_enabled(self, marker)
    }
    fn warn_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        AbstractLogger::warn_from_marker_with_msg(self, marker, msg);
    }
    fn warn_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        AbstractLogger::warn_from_marker_with_arg(self, marker, format, args);
    }
    fn warn_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        AbstractLogger::warn_from_marker_with_msg_and_error(self, marker, msg, t);
    }
    fn is_error_enabled(&self)->bool{
        RECORD_ALL_EVENTS
    }
    fn error_with_msg(&self, msg:String) {
        AbstractLogger::warn_with_msg(self, msg);
    }
    fn error_with_arg(&self, format:String, args:Vec<Argument>) {
        AbstractLogger::error_with_arg(self, format, args);
    }
    fn error_with_msg_and_error(&self, msg:String, t:Throwable) {
        AbstractLogger::error_with_msg_and_error(self, msg, t);
    }
    fn is_error_from_marker_enabled(&self, marker:&Box<dyn Marker>) -> bool {
        LegacyAbstractLogger::is_error_from_marker_enabled(self, marker)
    }
    fn error_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        AbstractLogger::error_from_marker_with_msg(self, marker, msg);
    }
    fn error_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<crate::utils::Argument>) {
        AbstractLogger::error_from_marker_with_arg(self, marker, format, args);
    }
    fn error_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        AbstractLogger::error_from_marker_with_msg_and_error(self, marker, msg, t);
    }
}