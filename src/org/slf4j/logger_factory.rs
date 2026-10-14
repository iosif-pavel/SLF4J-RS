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

use crate::org::slf4j::helpers::SubstituteServiceProvider;
pub enum LoggerInitialization {
    Uninitialized,
    OngoingInitialization,
    FailedInitialization,
    SeccessfulInitialization,
    NopFallbackInitialization,
}
pub struct LoggerFactory{
    initialization_state:LoggerInitialization,
    subst_provider:SubstituteServiceProvider,
}

//CONSTANTS
impl LoggerFactory {
    // Change the constants for these functions to facilitate the construction of the links and other compound messages
    fn codes_prefix()->String{
        "https://www.slf4j.org/codes.html".into()
    }
    fn no_providers_url()->String{
        Self::codes_prefix() + "#noProviders"
    }
    fn ignored_bindings_url()->String{
        Self::codes_prefix() + "#ignoredBindings"
    }
    fn multiple_bindings_url()->String{
        Self::codes_prefix() + "#multiple_bindings"
    }
    fn version_mismatch()->String{
        Self::codes_prefix() + "#version_mismatch"
    }
    fn substitute_logger_url()->String{
        Self::codes_prefix() + "#substituteLogger"
    }
    fn logger_name_mismatch_url() -> String{
        Self::codes_prefix() + "#loggerNameMismatch"
    }
    fn replay_url()->String{
        Self::codes_prefix() + "#replay"
    }
    fn unsuccessful_init_url()-> String{
        Self::codes_prefix() + "#unsuccessfulInit"
    }
    fn unsuccessful_init_msg() ->String{
        "org.slf4j.LoggerFactory in failed state. Original exception was thrown EARLIER. See also ".to_string() + Self::unsuccessful_init_url().as_str()
    }
    fn connected_with_msg() -> String{
        "Connected with provider of type [".into()
    }

    const PROVIDER_PROPERTY_KEY:&'static str = "slf4j.provider";

}