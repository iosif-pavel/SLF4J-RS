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
    org::slf4j::Marker, 
    utils::{
        Throwable,
        Argument,
        Value
    }
};

// Valor de CallerData por default para DEPTH
const DEFAULT_CALLER_DATA_DEPTH:u8 = 8;

pub trait LoggingEventBuilder {
    fn set_cause(&mut self, cause:Throwable);
    
    fn add_marker(&mut self, marker: &dyn Marker);

    // Implementa tanto la version normal como la Supplier
    // Asi como de los diferentes tipos
    fn add_argument(&mut self, arg:Argument);
    
    // Implementa tanto la version normal como la Supplier
    // Asi como de los diferentes tipos
    fn arg(&mut self, arg:Argument){
        self.add_argument(arg);
    }
    
    // Implementa tanto la version normal como la Supplier
    // Asi como de los diferentes tipos
    fn add_key_value(&mut self, key:String, value:Box<dyn Value>);

    // Implementa tanto la version normal como la Supplier
    // Asi como de los diferentes tipos
    fn kv(&mut self, key:String, value:Box<dyn Value>){
        self.add_key_value(key, value);
    }

    fn with_caller_data_depth(&mut self, depth:u8);
    
    fn with_caller_data_default(&mut self) {
        self.with_caller_data_depth(DEFAULT_CALLER_DATA_DEPTH)
    }

    fn set_message(&mut self, msg:String);

    fn set_message_from_supplier(&mut self, msg:dyn Fn()->String);

    // Equivale a log() de la interfaz original
    fn save_log(&mut self);

    // Equivale a llamar set_mesage seguido de save_log
    fn log_with_msg(&mut self, msg:String);
    
    // Equivale a llamar set_mesage seguido de add_argument y segido de save_log
    fn log_with_msg_and_arg(&mut self, msg:String, arg:Vec<Argument>);

    // Equivale a llamar a set_message_from_supplier seguido de save_log
    fn log_with_msg_from_supplier(&mut self, msg:dyn Fn()->String);
}