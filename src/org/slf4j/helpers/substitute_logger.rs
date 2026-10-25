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

use crate::org::slf4j::{Logger, SharedLogger};

 #[derive(Clone)]
pub struct SubstituteLogger;

impl SubstituteLogger {
    pub fn get_name(&self)->&String{
        todo!()
    }

    pub fn convert_to_arc(&self)->SharedLogger{
        let logger = self.clone();
        Arc::new(logger)
    }
}

impl Logger for SubstituteLogger {
    fn get_name(&self)->String {
        todo!()
    }

    fn clone_at(&self)->crate::org::slf4j::SharedLogger {
        todo!()
    }

    fn is_trace_enabled(&self)->bool {
        todo!()
    }

    fn trace_with_msg(&self, msg:String) {
        todo!()
    }

    fn trace_with_arg(&self, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn trace_with_msg_and_error(&self, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_trace_from_marker_enabled(&self, marker:&Box<dyn crate::org::slf4j::Marker>) -> bool {
        todo!()
    }

    fn trace_from_marker_with_msg(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String) {
        todo!()
    }

    fn trace_from_marker_with_arg(&self, marker:Box<dyn crate::org::slf4j::Marker>, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn trace_from_marker_with_msg_and_error(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_debug_enabled(&self)->bool {
        todo!()
    }

    fn debug_with_msg(&self, msg:String) {
        todo!()
    }

    fn debug_with_arg(&self, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn debug_with_msg_and_error(&self, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_debug_from_marker_enabled(&self, marker:&Box<dyn crate::org::slf4j::Marker>) -> bool {
        todo!()
    }

    fn debug_from_marker_with_msg(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String) {
        todo!()
    }

    fn debug_from_marker_with_arg(&self, marker:Box<dyn crate::org::slf4j::Marker>, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn debug_from_marker_with_msg_and_error(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_info_enabled(&self)->bool {
        todo!()
    }

    fn info_with_msg(&self, msg:String) {
        todo!()
    }

    fn info_with_arg(&self, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn info_with_msg_and_error(&self, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_info_from_marker_enabled(&self, marker:&Box<dyn crate::org::slf4j::Marker>) -> bool {
        todo!()
    }

    fn info_from_marker_with_msg(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String) {
        todo!()
    }

    fn info_from_marker_with_arg(&self, marker:Box<dyn crate::org::slf4j::Marker>, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn info_from_marker_with_msg_and_error(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_warn_enabled(&self)->bool {
        todo!()
    }

    fn warn_with_msg(&self, msg:String) {
        todo!()
    }

    fn warn_with_arg(&self, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn warn_with_msg_and_error(&self, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_warn_from_marker_enabled(&self, marker:&Box<dyn crate::org::slf4j::Marker>) -> bool {
        todo!()
    }

    fn warn_from_marker_with_msg(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String) {
        todo!()
    }

    fn warn_from_marker_with_arg(&self, marker:Box<dyn crate::org::slf4j::Marker>, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn warn_from_marker_with_msg_and_error(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_error_enabled(&self)->bool {
        todo!()
    }

    fn error_with_msg(&self, msg:String) {
        todo!()
    }

    fn error_with_arg(&self, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn error_with_msg_and_error(&self, msg:String, t:crate::utils::Throwable) {
        todo!()
    }

    fn is_error_from_marker_enabled(&self, marker:&Box<dyn crate::org::slf4j::Marker>) -> bool {
        todo!()
    }

    fn error_from_marker_with_msg(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String) {
        todo!()
    }

    fn error_from_marker_with_arg(&self, marker:Box<dyn crate::org::slf4j::Marker>, format:String, args:Vec<crate::utils::Argument>) {
        todo!()
    }

    fn error_from_marker_with_msg_and_error(&self, marker:Box<dyn crate::org::slf4j::Marker>, msg:String, t:crate::utils::Throwable) {
        todo!()
    }
}