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
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool);
        fn can_unwrap(&self) -> bool;
        fn unwrap_ok(self) -> O;
        fn unwrap_err(self) -> E;
    }

    impl DebugUnwrap<bool, bool> for bool {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool) { ("true", "false", false) }
        fn can_unwrap(&self) -> bool { *self }
        fn unwrap_ok(self) -> bool { false }
        fn unwrap_err(self) -> bool { false }
    }

    impl<T> DebugUnwrap<T,()> for Option<T> {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool) { ("Some", "None", false) }
        fn can_unwrap(&self) -> bool { self.is_some() }
        fn unwrap_ok(self) -> T { if let Some(r) = self { r } else { unreachable!() } }
        fn unwrap_err(self) -> () { assert!(self.is_none()); () }
    }

    impl<R,E: fmt::Debug> DebugUnwrap<R,E> for Result<R,E> {
        fn get_pass_fail_strs(&self) -> (&'static str, &'static str, bool) { ("Ok",   "Err", true) }
        fn can_unwrap(&self) -> bool { self.is_ok() }
        fn unwrap_ok(self) -> R { if let Ok(r) = self { r } else { unreachable!() } }
        fn unwrap_err(self) -> E { if let Err(e) = self { e } else { unreachable!() } }
    }

    fn output (mut message: String) {
        if !message.ends_with("\0") {
            message.push('\0');
        }

        #[allow(unused_unsafe)] unsafe {
            #[allow(unused_imports)] use crate::ffi::*;
            #[cfg(windows)] win32::OutputDebugStringA(message.as_ptr());
            #[cfg(android)] android::__android_log_write(android::Priority::ERROR, "bugsalot\0".as_ptr(), message.as_ptr());
            // TODO: Linux (stderr?)
            // TODO: Node (stderr?)
            // TODO: Browser (console.log)
        }
    }

    // TODO: Consider abusing const/static structs to minimize the amount of codegen we need at each call site just to initialize argument registers.
    pub fn log_unwrap_failed<O, E: fmt::Debug, DU: DebugUnwrap<O, E>>(file: &str, line: u32, msg: &str, expr: &str, du: DU) {
        let (pass, fail, format) = du.get_pass_fail_strs();
        let err = du.unwrap_err();
        if format {
            output(format!(
                concat!(
                    "{}({}): {}\r\n",
                    "    Expression: {}\r\n",
                    "    Expected:   {}(...)\r\n",
                    "    Found:      {}({:?})\r\n\0",
                ),
                file, line, msg,
                expr,
                pass,
                fail, MaybeDebugToDebug(&err)
            ));
        } else {
            output(format!(
                concat!(
                    "{}({}): {}\r\n",
                    "    Expression: {}\r\n",
                    "    Expected:   {}(...)\r\n",
                    "    Found:      {}\r\n\0",
                ),
                file, line, msg,
                expr,
                pass,
                fail
            ));
        }
    }
}

/// Unwraps Options and Results, logging/breaking on errors, but unlike `a.unwrap()` this is nonfatal and continuable.
/// 
/// Other differences:
/// * Works on booleans
/// * Should breakpoint directly on the line of the unwrap!
/// 
/// # Examples
/// 
/// ```
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

/// Unwraps Options and Results, logging/breaking on errors, but unlike `a.expect("msg")` this is nonfatal and continuable.
/// 
/// Other differences:
/// * Works on booleans
/// * Should breakpoint directly on the line of the unwrap!
/// 
/// # Examples
/// 
/// ```
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
