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

use std::{collections::VecDeque, time::SystemTime, thread::current};

use crate::{org::slf4j::{Marker, event::{Level, SubstituteLoggingEvent}, helpers::SubstituteLogger}, utils::{Argument, Throwable}};
pub const  SERIAL_VERSION_UID:i64 = -176083308134819629;
pub struct InternEventRecordingLogger{
    name:String,
    logger:SubstituteLogger,
    event_queue:VecDeque<SubstituteLoggingEvent>
}

impl InternEventRecordingLogger {
    pub fn new(
        logger:SubstituteLogger,
        event_queue:VecDeque<SubstituteLoggingEvent>
    )->Self{
        let name = logger.get_name().into();
        Self { name, logger, event_queue }
    }

    pub fn get_name(&self)->&String{
        &self.name
    }

    pub fn handleNormalizedLoggingCall(
        &mut self,
        level:Option<Level>,
        marker:Option<Box<dyn Marker>>,
        msg:Option<String>,
        args:Vec<Argument>,
        throwable:Option<Throwable>
    ){
        let mut logging_event = SubstituteLoggingEvent::new();
        logging_event.set_time_stamp({
            let now = SystemTime::now();
            let elapsed = now.elapsed().expect("Se esperaba una duracion");
            elapsed.as_millis() as i64
        });
        if level.is_some() {
            logging_event.set_level(level.unwrap());
        }else{
            logging_event.set_level(Level::Default);
        }
        logging_event.set_logger(self.logger.convert_to_arc());
        logging_event.set_logger_name(&self.name);
        if marker.is_some(){
            logging_event.add_marker(marker.unwrap());
        }
        logging_event.set_message(
            match msg {
                Some(msg) => msg,
                None => String::new()
            }
        );
        logging_event.set_thread_name({
            let thread = current();
            match thread.name() {
                Some(name) => name.into(),
                None => String::new()
            }
        });
        logging_event.set_args(args);
        logging_event.set_throwable(throwable);
        self.event_queue.push_back(logging_event);
    }

    pub fn get_fully_qualified_caller_name(&self)->String{
        String::new()
    }
}