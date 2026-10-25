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

use std::{any::Any};

use crate::{
    org::{
        slf4j::{
            Logger, 
            LoggerFactory, 
            Marker, 
            SharedLogger, 
            event::Level, 
            helpers::MessageFormatter
        }
    },
    utils::{
        Argument, 
        Throwable
    }
};

//pub static serial_version_uid:i64 = -2529255052481744503;

pub trait AbstractLogger: Logger {
    fn read_resolve(&self)-> SharedLogger{
        let factory = LoggerFactory::singleton();
        factory.get_logger(self.get_name().into())
    }
    
    //Helpers
    fn with_msg(&self, level:Level, marker:Option<Box<dyn Marker>>, msg:Option<String>){
        self.handle_0_args_call(Some(level), marker, msg, None)
    }
    fn with_args(&self, level:Level, marker:Option<Box<dyn Marker>>, format:String, args:Vec<Argument>){
        let len = args.len();
        match len {
            1 => {
                let arg = args[0].clone();
                self.handle_1_args_call(Some(level), marker, Some(format), arg);
            }
            2 => {
                let arg1 = args[0].clone();
                let arg2 = args[1].as_any();
                self.handle_2_args_call(Some(level), marker, Some(format), arg1, arg2);
            }
            _ => {
                let arg:Vec<Box<dyn Any>> = args.iter().map(|a| a.as_any()).collect();
                self.handle_arg_arrray_call(Some(level), marker, Some(format), arg);
            }
        }
    }
    fn with_msg_and_error(&self, level:Level, marker:Option<Box<dyn Marker>>, msg:Option<String>, t:Option<Throwable>){
        self.handle_0_args_call(Some(level), marker, msg, t);
    }

    // TRACE
    fn trace_with_msg(&self, msg:String) {
        if self.is_trace_enabled(){
            self.with_msg(Level::Trace, None, Some(msg));
        }
    }
    fn trace_with_arg(&self, format:String, args:Vec<Argument>) {
        if self.is_trace_enabled() {
            self.with_args(Level::Trace, None, format, args);
        }
    }
    fn trace_with_msg_and_error(&self, msg:String, t:Throwable) {
        if self.is_trace_enabled(){
            self.with_msg_and_error(Level::Trace, None, Some(msg), Some(t));
        }
    }
    fn trace_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        if self.is_trace_from_marker_enabled(&marker){
            self.with_msg(Level::Trace, Some(marker), Some(msg));
        }
    }
    fn trace_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        if self.is_trace_enabled() {
            self.with_args(Level::Trace, Some(marker), format, args);
        }
    }
    fn trace_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        if self.is_trace_from_marker_enabled(&marker){
            self.with_msg_and_error(Level::Trace, Some(marker), Some(msg), Some(t));
        }
    }
    
    // DEBUG
    fn debug_with_msg(&self, msg:String) {
        if self.is_debug_enabled() {
            self.with_msg(Level::Debug, None, Some(msg))
        }
    }
    fn debug_with_arg(&self, format:String, args:Vec<Argument>) {
        if self.is_debug_enabled() {
            self.with_args(Level::Debug, None, format, args);
        }
    }
    fn debug_with_msg_and_error(&self, msg:String, t:Throwable) {
        if self.is_debug_enabled(){
            self.with_msg_and_error(Level::Debug, None, Some(msg), Some(t));
        }
    }
    fn debug_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        if self.is_debug_from_marker_enabled(&marker){
            self.with_msg(Level::Debug, Some(marker), Some(msg));
        }
    }
    fn debug_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        if self.is_debug_enabled() {
            self.with_args(Level::Debug, Some(marker), format, args);
        }
    }
    fn debug_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        if self.is_debug_from_marker_enabled(&marker){
            self.with_msg_and_error(Level::Debug, Some(marker), Some(msg), Some(t));
        }
    }
    
    // INFO
    fn info_with_msg(&self, msg:String) {
        if self.is_info_enabled() {
            self.with_msg(Level::Info, None, Some(msg));
        }
    }
    fn info_with_arg(&self, format:String, args:Vec<Argument>) {
        if self.is_info_enabled() {
            self.with_args(Level::Info, None, format, args);
        }
    }
    fn info_with_msg_and_error(&self, msg:String, t:Throwable) {
        if self.is_info_enabled() {
            self.with_msg_and_error(Level::Info, None, Some(msg), Some(t));
        }
    }
    fn info_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        if self.is_info_from_marker_enabled(&marker){
            self.with_msg(Level::Info, Some(marker), Some(msg));
        }
    }
    fn info_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        if self.is_info_from_marker_enabled(&marker){
            self.with_args(Level::Info, Some(marker), format, args);
        }
    }
    fn info_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        if self.is_info_from_marker_enabled(&marker){
            self.with_msg_and_error(Level::Info, Some(marker), Some(msg), Some(t));
        }
    }
    
    // WARN
    fn warn_with_msg(&self, msg:String) {
        if self.is_warn_enabled(){
            self.with_msg(Level::Warn, None, Some(msg));
        }
    }
    fn warn_with_arg(&self, format:String, args:Vec<Argument>) {
        if self.is_warn_enabled(){
            self.with_args(Level::Warn, None, format, args);
        }
    }
    fn warn_with_msg_and_error(&self, msg:String, t:Throwable) {
        if self.is_warn_enabled(){
            self.with_msg_and_error(Level::Warn, None, Some(msg), Some(t));
        }
    }
    fn warn_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        if self.is_warn_from_marker_enabled(&marker){
            self.with_msg(Level::Warn, Some(marker), Some(msg));
        }
    }
    fn warn_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        if self.is_warn_from_marker_enabled(&marker){
            self.with_args(Level::Warn, Some(marker), format, args);
        }
    }
    fn warn_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        if self.is_warn_from_marker_enabled(&marker) {
            self.with_msg_and_error(Level::Warn, Some(marker), Some(msg), Some(t));
        }
    }
    
    //ERROR
    fn error_with_msg(&self, msg:String) {
        if self.is_error_enabled() {
            self.with_msg(Level::Error, None, Some(msg));
        }
    }
    fn error_with_arg(&self, format:String, args:Vec<Argument>) {
        if self.is_error_enabled() {
            self.with_args(Level::Error, None, format, args);
        }
    }
    fn error_with_msg_and_error(&self, msg:String, t:Throwable) {
        if self.is_error_enabled() {
            self.with_msg_and_error(Level::Error, None, Some(msg), Some(t));
        }
    }
    fn error_from_marker_with_msg(&self, marker:Box<dyn Marker>, msg:String) {
        if self.is_error_from_marker_enabled(&marker){
            self.with_msg(Level::Error, Some(marker), Some(msg));
        }
    }
    fn error_from_marker_with_arg(&self, marker:Box<dyn Marker>, format:String, args:Vec<Argument>) {
        if self.is_error_from_marker_enabled(&marker){
            self.with_args(Level::Error, Some(marker), format, args);
        }
    }
    fn error_from_marker_with_msg_and_error(&self, marker:Box<dyn Marker>, msg:String, t:Throwable) {
        if self.is_error_from_marker_enabled(&marker){
            self.with_msg_and_error(Level::Error, Some(marker), Some(msg), Some(t));
        }
    }
    
    // HANDLERS
    fn handle_0_args_call(
        &self,
        level:Option<Level>, 
        marker:Option<Box<dyn Marker>>,
        msg:Option<String>,
        t:Option<Throwable>
    ){
        self.handle_normalized_logging_call(
            level,
            marker,
            msg,
            None,
            t
        )
    }
    fn handle_1_args_call(
        &self,
        level:Option<Level>,
        marker:Option<Box<dyn Marker>>,
        msg:Option<String>,
        arg1:Argument,
    ){
        self.handle_normalized_logging_call(
            level, 
            marker, 
            msg, 
            Some(vec![arg1]), 
            None
        );
    }
    fn handle_2_args_call(
        &self,
        level:Option<Level>,
        marker:Option<Box<dyn Marker>>,
        msg:Option<String>,
        arg1:Argument,
        arg2:Box<dyn Any>
    ){
        if arg2.is::<Throwable>(){
            let arg2 = arg2.downcast::<Throwable>().expect("Deveria de dar un resultado");
            self.handle_normalized_logging_call(
                level, 
                marker, 
                msg, 
                Some(vec![arg1]), 
                Some(*arg2)
            );
        } else if arg2.is::<Argument>(){
            let arg = arg2.downcast::<Argument>().expect("Deveria de dar un resultado");
            self.handle_normalized_logging_call(
                level, 
                marker, 
                msg, 
                Some(vec![arg1, *arg]), 
                None
            );
        }
    }
    fn handle_arg_arrray_call(
        &self,
        level:Option<Level>,
        marker:Option<Box<dyn Marker>>,
        msg:Option<String>,
        args:Vec<Box<dyn Any>>
    ){
        let throwable_candidate = MessageFormatter::get_throwable_candidate(&args);
        if throwable_candidate.is_some(){
            let trimmed_copy = MessageFormatter::trimmed_copy(&args);
            self.handle_normalized_logging_call(
                level, 
                marker, 
                msg, 
                trimmed_copy, 
                throwable_candidate
            );
        }else {
            self.handle_normalized_logging_call(
                level, 
                marker, 
                msg, 
                Some(
                    args.iter().map(|a|{
                        if let Some(arg) = a.downcast_ref::<Argument>(){
                            let arg = arg.clone();
                            arg
                        }else {
                            Argument::new_void() // Los voids se ignoraran en procesos posteriores
                        }
                    })
                    .collect()
                ),
                None
            );
        }
    }
    fn get_full_qualified_caller_name(&self)->&String;
    fn handle_normalized_logging_call(
        &self, 
        level:Option<Level>, 
        marker:Option<Box<dyn Marker>>, 
        msg:Option<String>, 
        args:Option<Vec<Argument>>, 
        t:Option<Throwable>
    );
}