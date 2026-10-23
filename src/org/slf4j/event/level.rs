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

#[derive(PartialEq)]
pub struct Level{
    level_int:u8,
    level_str:&'static str,
}

impl Level {
    const fn new(i:u8, s:&'static str)->Self{
        Self { level_int: i, level_str: s }
    }
    pub const ERROR:Level=Self::new(EventConstants::ERROR, "ERROR");
    pub const WARN:Level=Self::new(EventConstants::WARN, "WARN");
    pub const INFO:Level=Self::new(EventConstants::INFO, "INFO");
    pub const DEBUG:Level=Self::new(EventConstants::DEBUG, "DEBUG");
    pub const TRACE:Level=Self::new(EventConstants::TRACE, "TRACE");

    pub fn to_int(&self)->u8{
        self.level_int
    }

    pub fn int_to_level(int:u8)->Option<Self>{
        Some(
            match int {
                EventConstants::TRACE => Self::TRACE,
                EventConstants::DEBUG => Self::DEBUG,
                EventConstants::INFO => Self::INFO,
                EventConstants::WARN => Self::WARN,
                EventConstants::ERROR => Self::ERROR,
                _ => return None
            }
        )
    }
}

impl Debug for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.level_str)
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.level_str)
    }
}