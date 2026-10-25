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

use std::fmt::{Debug, Display};

use crate::org::slf4j::{
    event::{EventConstants}, spi::LocationAwareLoggerConstants
};
#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Level {
    Error = EventConstants::ERROR,
    Warn = EventConstants::WARN,
    Info = EventConstants::INFO,
    Debug = EventConstants::DEBUG,
    Trace = EventConstants::TRACE
}

impl Level {
    
    pub fn to_int(&self)->u8{
        *self as u8
    }

    pub fn int_to_level(int:u8)->Option<Self>{
        Some(
            match int {
                EventConstants::TRACE => Self::Trace,
                EventConstants::DEBUG => Self::Debug,
                EventConstants::INFO => Self::Info,
                EventConstants::WARN => Self::Warn,
                EventConstants::ERROR => Self::Error,
                _ => return None
            }
        )
    }
}

impl Debug for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Level::Error => write!(f, "ERROR"),
            Level::Warn => write!(f, "WARN"),
            Level::Info => write!(f, "INFO"),
            Level::Debug => write!(f, "DEBUG"),
            Level::Trace => write!(f, "TRACE"),
        }
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}