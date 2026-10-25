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

use crate::org::slf4j::{
    Logger, Marker, helpers::AbstractLogger
};

pub static SERIAL_VERSION_UID:i64 = -7041884104854048950;

pub trait LegacyAbstractLogger:AbstractLogger + Logger {
    fn is_trace_from_marker_enabled(&self, _marker:&Box<dyn Marker>) -> bool {
        self.is_trace_enabled()
    }
    fn is_debug_from_marker_enabled(&self, _marker:&Box<dyn Marker>) -> bool {
        self.is_debug_enabled()
    }
    fn is_info_from_marker_enabled(&self, _marker:&Box<dyn Marker>) -> bool {
        self.is_info_enabled()
    }
    fn is_warn_from_marker_enabled(&self, _marker:&Box<dyn Marker>) -> bool {
        self.is_warn_enabled()
    }
    fn is_error_from_marker_enabled(&self, _marker:&Box<dyn Marker>) -> bool {
        self.is_error_enabled()
    }
}