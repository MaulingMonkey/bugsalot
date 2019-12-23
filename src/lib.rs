// TODO: Module level docs.

mod ffi;
pub mod debugger;

#[doc(hidden)] pub mod macro_impl {
    use std::fmt::{self, Debug, Display, Formatter};

    pub trait MaybeDebug { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt("???", f) } }
    impl<T: Debug> MaybeDebug for T { fn fmt(&self, f: &mut Formatter) -> fmt::Result { self.fmt(f) } }

    struct MaybeDebugToDebug<'a>(&'a dyn MaybeDebug);
    impl<'a> Debug for MaybeDebugToDebug<'a> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { self.0.fmt(f) } }

    pub trait DebugUnwrap<O, E : fmt::Debug> {
        /// Returns (pass, fail, format_error)
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool, bool);
        fn can_unwrap(&self) -> bool;
        fn unwrap_ok(self) -> O;
        fn unwrap_err(self) -> E;
    }

    impl DebugUnwrap<bool, bool> for bool {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool, bool) { ("true", "false", false, false) }
        fn can_unwrap(&self) -> bool { *self }
        fn unwrap_ok(self) -> bool { false }
        fn unwrap_err(self) -> bool { false }
    }

    impl<T> DebugUnwrap<*const T, *const T> for *const T {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool, bool) { ("non-null", "null", false, false) }
        fn can_unwrap(&self) -> bool { *self != 0 as *const T }
        fn unwrap_ok(self) -> *const T { self }
        fn unwrap_err(self) -> *const T { self }
    }

    impl<T> DebugUnwrap<*mut T, *mut T> for *mut T {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool, bool) { ("non-null", "null", false, false) }
        fn can_unwrap(&self) -> bool { *self != 0 as *mut T }
        fn unwrap_ok(self) -> *mut T { self }
        fn unwrap_err(self) -> *mut T { self }
    }

    impl<T> DebugUnwrap<T,()> for Option<T> {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool, bool) { ("Some", "None", true, false) }
        fn can_unwrap(&self) -> bool { self.is_some() }
        fn unwrap_ok(self) -> T { if let Some(r) = self { r } else { unreachable!() } }
        fn unwrap_err(self) -> () { assert!(self.is_none()); () }
    }

    impl<R,E: fmt::Debug> DebugUnwrap<R,E> for Result<R,E> {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool, bool) { ("Ok",   "Err", false, true) }
        fn can_unwrap(&self) -> bool { self.is_ok() }
        fn unwrap_ok(self) -> R { if let Ok(r) = self { r } else { unreachable!() } }
        fn unwrap_err(self) -> E { if let Err(e) = self { e } else { unreachable!() } }
    }

    #[allow(dead_code)]
    fn require_nul(mut message: String) -> String {
        if !message.ends_with("\0") { message.push('\0'); }
        message
    }

    #[allow(dead_code)]
    fn require_no_nul(mut message: String) -> String {
        if message.ends_with("\0") { message.pop(); }
        message
    }

    pub fn output(message: String) {
        #[allow(unused_imports)] use crate::ffi::*;
        #[allow(unused_unsafe)] unsafe {
            #[cfg(windows)] win32::OutputDebugStringA(require_nul(message).as_ptr());
            #[cfg(target_os = "android")] android::__android_log_write(android::Priority::ERROR, "bugsalot\0".as_ptr(), require_nul(message).as_ptr());
        }

        #[cfg(target_arch = "wasm32")] wasm::console::error(require_no_nul(message));
        #[cfg(all(unix, not(target_os = "android")))] eprint!("{}", require_no_nul(message));
    }

    // TODO: Consider abusing const/static structs to minimize the amount of codegen we need at each call site just to initialize argument registers.
    pub fn log_unwrap_failed<M: std::fmt::Display, O, E: fmt::Debug, DU: DebugUnwrap<O, E>>(file: &str, line: u32, msg: M, expr: &str, du: DU) {
        let (pass, fail, pass_parens, fail_parens) = du.get_pass_fail_strs();
        let err = du.unwrap_err();
        if fail_parens {
            output(format!(
                concat!(
                    "{}({}): {}\r\n",
                    "    Expression: {}\r\n",
                    "    Expected:   {}{}\r\n",
                    "    Found:      {}({:?})\r\n\0",
                ),
                file, line, msg,
                expr,
                pass, if pass_parens { "(...)" } else { "" },
                fail, MaybeDebugToDebug(&err)
            ));
        } else {
            output(format!(
                concat!(
                    "{}({}): {}\r\n",
                    "    Expression: {}\r\n",
                    "    Expected:   {}{}\r\n",
                    "    Found:      {}\r\n\0",
                ),
                file, line, msg,
                expr,
                pass, if pass_parens { "(...)" } else { "" },
                fail
            ));
        }
    }

    pub fn log_bug(file: &str, line: u32, msg: impl std::fmt::Display) {
        output(format!(
            "{}({}): {}\r\n",
            file, line, msg
        ));
    }
}

/// Reports a bug by logging/breaking.  Unlike `panic!(...)` this is nonfatal and continuable.
/// 
/// # Examples
/// 
/// ```no_run
/// use bugsalot::bug;
/// 
/// bug!();
/// bug!("A simple bug expression, {} allowed");
/// bug!("A formatting bug expression: {}", "automatically wrapped in format!(...)");
/// ```
#[macro_export]
macro_rules! bug {
    ( $e:expr ) => {{
        $crate::macro_impl::log_bug(file!(), line!(), $e);
        $crate::debugger::break_if_attached();
    }};
    ()              => { $crate::bug!("bug!()") };
    ( $($tt:tt)+ )  => { $crate::bug!(format!($($tt)+)) };
}

/// Log (part of) a line to standard debugging channels.  **Prefer [debugln!]**
/// 
/// Messages should terminate in newlines.  Failure to do so will result in inconsistent behavior between debug message
/// viewers, as several of them treat each message as a complete package and effectively terminate them with newlines
/// even if you don't, but a few *don't*, and will instead effectively combine lines.
/// 
/// For example, in [DebugView] on Windows, `debug!("A"); debug!("B");` will display something like:
/// 
/// ```text
/// 33  616.02484131 [12584] A
/// 34  616.02490234 [12584] B
/// ```
/// 
/// But in [Visual Studio]'s Output tab, or [Visual Studio Code]'s Debug Console, it will instead display something like:
/// 
/// ```text
/// AB
/// ```
/// 
/// # Examples
/// 
/// ```
/// use bugsalot::debug;
/// 
/// debug!("Print stuff\n");
/// debug!("Print more stuff: {} {} {}\n", 1, "2", 3.0f32);
/// ```
/// 
/// # Platforms
/// 
/// | platform  | mechanism | notes and caveats |
/// | --------- | --------- | ----------------- |
/// | Android   | [__android_log_write](https://developer.android.com/ndk/reference/group/logging#group___logging_1ga32a7173b092ec978b50490bd12ee523b)  | View in [ADB Logcat].
/// | WASM      | [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)                                                           | View in Developer Tools ([Firefox], [Chrome]).
/// | Windows   | [OutputDebugStringA](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringa)                      | **Buffer size capped.**  View in [VS]'s Ouput tab, [VSC]'s Debug Console, [DebugView](https://docs.microsoft.com/en-us/sysinternals/downloads/debugview), etc.
/// | \*nix     | stderr                                                                                                                                | May intermingle with other error reporting, subject to change.
/// 
/// [debugln!]:             macro.debugln.html
/// [ADB Logcat]:           https://developer.android.com/studio/command-line/logcat
/// [Firefox]:              https://developer.mozilla.org/en-US/docs/Tools/Web_Console
/// [Chrome]:               https://developers.google.com/web/tools/chrome-devtools/console#view
/// [DebugView]:            https://docs.microsoft.com/en-us/sysinternals/downloads/debugview
/// [VS]:                   https://visualstudio.microsoft.com/
/// [Visual Studio]:        https://visualstudio.microsoft.com/
/// [VSC]:                  https://code.visualstudio.com/
/// [Visual Studio Code]:   https://code.visualstudio.com/
#[macro_export]
macro_rules! debug {
    ( $format:literal $($tt:tt)* )  => { $crate::macro_impl::output(format!($format $($tt)*)) };
}

/// Log a line to standard debugging channels.
/// 
/// # Examples
/// 
/// ```
/// use bugsalot::debugln;
/// 
/// debugln!();
/// debugln!("Print stuff");
/// debugln!("Print more stuff: {} {} {}", 1, "2", 3.0f32);
/// ```
/// 
/// # Platforms
/// 
/// | platform  | mechanism | notes and caveats |
/// | --------- | --------- | ----------------- |
/// | Android   | [__android_log_write](https://developer.android.com/ndk/reference/group/logging#group___logging_1ga32a7173b092ec978b50490bd12ee523b)  | View in [ADB Logcat].
/// | WASM      | [console.log](https://developer.mozilla.org/en-US/docs/Web/API/Console/log)                                                           | View in Developer Tools ([Firefox], [Chrome]).
/// | Windows   | [OutputDebugStringA](https://docs.microsoft.com/en-us/windows/win32/api/debugapi/nf-debugapi-outputdebugstringa)                      | **Buffer size capped.**  View in [VS]'s Ouput tab, [VSC]'s Debug Console, [DebugView](https://docs.microsoft.com/en-us/sysinternals/downloads/debugview), etc.
/// | \*nix     | stderr                                                                                                                                | May intermingle with other error reporting, subject to change.
/// 
/// [debugln!]:             macro.debugln.html
/// [ADB Logcat]:           https://developer.android.com/studio/command-line/logcat
/// [Firefox]:              https://developer.mozilla.org/en-US/docs/Tools/Web_Console
/// [Chrome]:               https://developers.google.com/web/tools/chrome-devtools/console#view
/// [DebugView]:            https://docs.microsoft.com/en-us/sysinternals/downloads/debugview
/// [VS]:                   https://visualstudio.microsoft.com/
/// [VSC]:                  https://code.visualstudio.com/
#[macro_export]
macro_rules! debugln {
    ()                              => { $crate::debugln!("") };
    ( $format:literal $($tt:tt)* )  => {
        #[cfg(not(windows))] { $crate::macro_impl::output(format!(concat!($format, "\n\0") $($tt)*))      }
        #[cfg(    windows )] { $crate::macro_impl::output(format!(concat!($format, "\r\n\0") $($tt)*))    }
    };
}

/// Unwraps Options and Results, logging/breaking on errors, but unlike `a.unwrap()` this is nonfatal and continuable.
/// 
/// Other differences:
/// * Works on booleans
/// * Should breakpoint directly on the line of the unwrap!
/// 
/// # Examples
/// 
/// ```no_run
/// use bugsalot::unwrap;
/// 
/// let a = true;
/// let _ : bool = unwrap!(a, false);
/// let _ : ()   = unwrap!(a, ());
/// let _ : ()   = unwrap!(a);
/// let _ : bool = unwrap!(a, return);
/// 
/// let a : Option<i32> = Some(42);
/// let _ : i32 = unwrap!(a, 0);
/// let _ : ()  = unwrap!(a, ());
/// let _ : ()  = unwrap!(a);
/// let _ : i32 = unwrap!(a, return);
/// 
/// let a : Result<i32, &'static str> = Ok(42);
/// let _ : i32 = unwrap!(a, 0);
/// let _ : ()  = unwrap!(a, ());
/// let _ : ()  = unwrap!(a);
/// let _ : i32 = unwrap!(a, return);
/// 
/// let a : *const i32 = &42;
/// let _ : i32 = unsafe { *unwrap!(a, &12) };
/// let _ : ()  =           unwrap!(a, ());
/// let _ : ()  =           unwrap!(a);
/// let _ : i32 = unsafe { *unwrap!(a, return) };
/// ```
#[macro_export]
macro_rules! unwrap {
    ( $e:expr, () ) => {{
        let unwrap_target = $e;
        if $crate::macro_impl::DebugUnwrap::can_unwrap(&unwrap_target) {
            $crate::macro_impl::DebugUnwrap::unwrap_ok(unwrap_target);
        } else {
            $crate::macro_impl::log_unwrap_failed(file!(), line!(), "unwrap! failed", stringify!($e), unwrap_target);
            $crate::debugger::break_if_attached();
        }
    }};

    ( $e:expr, $fallback:expr ) => {{
        let unwrap_target = $e;
        if $crate::macro_impl::DebugUnwrap::can_unwrap(&unwrap_target) {
            $crate::macro_impl::DebugUnwrap::unwrap_ok(unwrap_target)
        } else {
            $crate::macro_impl::log_unwrap_failed(file!(), line!(), "unwrap! failed", stringify!($e), unwrap_target);
            $crate::debugger::break_if_attached();
            $fallback
        }
    }};

    ( $e:expr ) => {{
        let unwrap_target = $e;
        if $crate::macro_impl::DebugUnwrap::can_unwrap(&unwrap_target) {
            $crate::macro_impl::DebugUnwrap::unwrap_ok(unwrap_target);
        } else {
            $crate::macro_impl::log_unwrap_failed(file!(), line!(), "unwrap! failed", stringify!($e), unwrap_target);
            $crate::debugger::break_if_attached();
        }
    }};
}

#[test]
fn unwrap_examples() {
    use crate::unwrap;

    let a = true;
    let _ : bool = unwrap!(a, false);
    let _ : ()   = unwrap!(a, ());
    let _ : ()   = unwrap!(a);
    let _ : bool = unwrap!(a, return);

    let a : Option<i32> = Some(42);
    let _ : i32 = unwrap!(a, 0);
    let _ : ()  = unwrap!(a, ());
    let _ : ()  = unwrap!(a);
    let _ : i32 = unwrap!(a, return);

    let a : Result<i32, &'static str> = Ok(42);
    let _ : i32 = unwrap!(a, 0);
    let _ : ()  = unwrap!(a, ());
    let _ : ()  = unwrap!(a);
    let _ : i32 = unwrap!(a, return);

    let a : *const i32 = &42;
    let _ : i32 = unsafe { *unwrap!(a, &12) };
    let _ : ()  =           unwrap!(a, ());
    let _ : ()  =           unwrap!(a);
    let _ : i32 = unsafe { *unwrap!(a, return) };
}

/// Unwraps Options and Results, logging/breaking on errors, but unlike `a.expect("msg")` this is nonfatal and continuable.
/// 
/// Other differences:
/// * Works on booleans
/// * Should breakpoint directly on the line of the unwrap!
/// 
/// # Examples
/// 
/// ```no_run
/// use bugsalot::expect;
/// 
/// let a = true;
/// let _ : bool = expect!(a, "Couldn't do something", false);
/// let _ : ()   = expect!(a, "Couldn't do something", ());
/// let _ : ()   = expect!(a, "Couldn't do something");
/// let _ : bool = expect!(a, "Couldn't do something", return);
/// 
/// let a : Option<i32> = Some(42);
/// let _ : i32 = expect!(a, "Couldn't do something", 0);
/// let _ : ()  = expect!(a, "Couldn't do something", ());
/// let _ : ()  = expect!(a, "Couldn't do something");
/// let _ : i32 = expect!(a, "Couldn't do something", return);
/// 
/// let a : Result<i32, &'static str> = Ok(42);
/// let _ : i32 = expect!(a, "Couldn't do something", 0);
/// let _ : ()  = expect!(a, "Couldn't do something", ());
/// let _ : ()  = expect!(a, "Couldn't do something");
/// let _ : i32 = expect!(a, "Couldn't do something", return);
/// 
/// let a : *const i32 = &42;
/// let _ : i32 = unsafe { *expect!(a, "Couldn't do something!", &12) };
/// let _ : ()  =           expect!(a, "Couldn't do something!", ());
/// let _ : ()  =           expect!(a,  format!("String {}", 42));
/// let _ : i32 = unsafe { *expect!(a, &format!("String {}", 42), return) };
/// ```
#[macro_export]
macro_rules! expect {
    ( $e:expr, $message:expr, () ) => {{
        let unwrap_target = $e;
        if $crate::macro_impl::DebugUnwrap::can_unwrap(&unwrap_target) {
            $crate::macro_impl::DebugUnwrap::unwrap_ok(unwrap_target);
        } else {
            $crate::macro_impl::log_unwrap_failed(file!(), line!(), $message, stringify!($e), unwrap_target);
            $crate::debugger::break_if_attached();
        }
    }};

    ( $e:expr, $message:expr, $err:expr ) => {{
        let unwrap_target = $e;
        if $crate::macro_impl::DebugUnwrap::can_unwrap(&unwrap_target) {
            $crate::macro_impl::DebugUnwrap::unwrap_ok(unwrap_target)
        } else {
            $crate::macro_impl::log_unwrap_failed(file!(), line!(), $message, stringify!($e), unwrap_target);
            $crate::debugger::break_if_attached();
            $err
        }
    }};

    ( $e:expr, $message:expr ) => {{
        let unwrap_target = $e;
        if $crate::macro_impl::DebugUnwrap::can_unwrap(&unwrap_target) {
            $crate::macro_impl::DebugUnwrap::unwrap_ok(unwrap_target);
        } else {
            $crate::macro_impl::log_unwrap_failed(file!(), line!(), $message, stringify!($e), unwrap_target);
            $crate::debugger::break_if_attached();
        }
    }};
}

#[test]
fn expect_examples() {
    use crate::expect;

    let a = true;
    let _ : bool = expect!(a, "Couldn't do something", false);
    let _ : ()   = expect!(a, "Couldn't do something", ());
    let _ : ()   = expect!(a, "Couldn't do something");
    let _ : bool = expect!(a, "Couldn't do something", return);

    let a : Option<i32> = Some(42);
    let _ : i32 = expect!(a, "Couldn't do something", 0);
    let _ : ()  = expect!(a, "Couldn't do something", ());
    let _ : ()  = expect!(a, "Couldn't do something");
    let _ : i32 = expect!(a, "Couldn't do something", return);

    let a : Result<i32, &'static str> = Ok(42);
    let _ : i32 = expect!(a, "Couldn't do something", 0);
    let _ : ()  = expect!(a, "Couldn't do something", ());
    let _ : ()  = expect!(a, "Couldn't do something");
    let _ : i32 = expect!(a, "Couldn't do something", return);

    let a : *const i32 = &42;
    let _ : i32 = unsafe { *expect!(a, "Couldn't do something!", &12) };
    let _ : ()  =           expect!(a, "Couldn't do something!", ());
    let _ : ()  =           expect!(a,  format!("String {}", 42));
    let _ : i32 = unsafe { *expect!(a, &format!("String {}", 42), return) };
}
