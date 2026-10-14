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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Level{
    level_int:u8,
    level_str:&'static str,
}

impl Level {
    const fn new(level_int:u8, level_str:&'static str)->Self{
        Self { level_int, level_str }
    }

    pub const ERROR:Level = Level::new(40, "ERROR");
    pub const WARN:Level = Level::new(30, "WARN");
    pub const INFO:Level = Level::new(20, "INFO");
    pub const DEBUG:Level = Level::new(10, "DEBUG");
    pub const TRACE:Level = Level::new(0, "TRACE");

    pub fn to_int(&self)->u8{
        self.level_int
    }

    pub fn int_to_level(i:u8)->Option<Level>{
        Some(match i {
             0 => Level::TRACE,
            10 => Level::DEBUG,
            20 => Level::INFO,
            30 => Level::WARN,
            40 => Level::ERROR,
            _ => return None
        })
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.level_str)
    }
}