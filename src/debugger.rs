//! APIs for interacting with native debuggers for the current process.
//! 
//! This includes:
//! 
//! * [Visual Studio](https://visualstudio.microsoft.com/vs/)
//! * [VS Code](https://code.visualstudio.com/)
//! * [WinDBG](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/debugger-download-tools)
//! * [CDB](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/debugger-download-tools)
//! * [ADPlus](https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/adplus)
//! * [GDB](https://www.gnu.org/software/gdb/)
//! * [LLDB](https://lldb.llvm.org/)
//! 
//! This is *not* intended to include:
//! 
//! * Script-only debuggers
//! * GPU/shader debuggers
//! * Java-only debuggers like `jdb`
//! * Stealthy reverse engineering emulators

/// Describes the possible states of the debugger: Detatched, Attached, or Unknown
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// No debugger is currently attached.
    Detatched,

    /// A debugger is currently attached.
    Attached,

    /// A debugger may or may not be attached.  Either this platform doesn't expose a means of testing, or that means
    /// is currently unavailable, or this library hasn't implemented the use of that API.
    Unknown,
}

/// What is the current state of the debugger, with regards to this process?  Attached?  Detatched?  Unknown?
/// 
/// # Platforms
/// 
/// | Platform  | State | Notes |
/// | --------- | ----- | ----- |
/// | Windows   | OK    |       |
/// | Android   | OK    |       |
/// | Linux     | OK    | Some versions of Windows Subsystem for Linux incorrectly report `Detatched`
/// | FreeBSD   | OK?   | Untested
/// | NetBSD    | OK?   | Untested
/// | OS X      | OK?   | Untested
/// | iOS       | OK?   | Untested
/// | WASM      | N/A   | Returns `Unknown`.  We could try to detect if devtools are open, but that's a browser specific mess, and you can usually get away with just invoking `debugger;`.  That said, pull requests welcome.
/// 
/// # KNOWN BUGS:
///
/// * On some versions of Windows Subsystem for Linux, `TracerPid` may incorrectly be 0 when a debugger is attached,
///   leading us to incorrectly report `Detatched`.  If you encounter such a version, please
///   [file an issue](https://github.com/MaulingMonkey/bugsalot/issues/new) that includes the result of
///   `cat /proc/version`, so I can add version detection to report `Unknown` for affected versions.
/// 
/// # Examples
/// 
/// ```
/// use bugsalot::debugger;
/// 
/// match debugger::state() {
///     debugger::State::Detatched  => println!("No debugger attached"),
///     debugger::State::Attached   => println!("A debugger is attached"),
///     debugger::State::Unknown    => println!("A debugger may or may not be attached"),
/// }
/// 
/// if cfg!(any(windows, target_os = "android", target_os = "linux")) {
///     // Platform should be supported
///     assert_ne!(debugger::state(), debugger::State::Unknown)
/// } else if cfg!(any(target_arch = "wasm")) {
///     // Platform is *not* supported
///     assert_eq!(debugger::state(), debugger::State::Unknown)
/// } else {
///     // Platform might be supported?  Might not?
/// }
/// ```
#[allow(unreachable_code)]
pub fn state () -> State {
    // For windows based platforms, just rely on Kernel32
    #[cfg(windows)] unsafe {
        return if crate::ffi::win32::IsDebuggerPresent() == 0 { State::Detatched } else { State::Attached }
    }

    // "/proc/self/status" may contain a TracerPid: [debugger process id] line, which is nonzero if there is a debugger.
    // Works on android, linux, and possibly on various BSDs and OS X.
    #[cfg(unix)] {
        // The following `/proc/version`s of WSL correctly report `TracerPid`: 
        // Linux version 4.4.0-18362-Microsoft (Microsoft@Microsoft.com) (gcc version 5.4.0 (GCC) ) #1-Microsoft Mon Mar 18 12:02:00 PST 2019
        // XXX: Do we maybe want to cache the result in a thread_local and/or static somewhere?
        if let Ok(file) = std::fs::File::open("/proc/self/status") {
            let file = std::io::BufReader::new(file);
            use std::io::BufRead;
            for line in file.lines() {
                if let Ok(line) = line {
                    let line = line.trim();
                    if line.starts_with("TracerPid:") {
                        return match [" 0", "\t0", ":0"].iter().any(|zero| line.ends_with(zero)) {
                            true  => State::Detatched,
                            false => State::Attached,
                        }
                    }
                }
            }
        }
    }

    State::Unknown
}

/// If a debugger is attached, breakpoint here.
/// 
/// # Platforms
/// 
/// | Platform  | State | Notes |
/// | --------- | ----- | ----- |
/// | Windows   | OK    |       |
/// | Android   | OK    |       |
/// | Linux     | OK    | See `state()` for known bugs, signal type might be wrong/suboptimal for debuggers.
/// | FreeBSD   | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | NetBSD    | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | OS X      | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | iOS       | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | WASM      | OK    |       |
/// 
/// # Examples
/// 
/// ```no_run
/// use bugsalot::debugger;
/// 
/// debugger::break_if_attached();
/// ```
#[inline(always)] // We'd strongly prefer if the debugger showed us the call site, not this function.
pub fn break_if_attached() {
    #[cfg(windows)] unsafe {
        if crate::ffi::win32::IsDebuggerPresent() != 0 {
            crate::ffi::win32::DebugBreak();
        }
        return;
    }

    #[cfg(target_arch = "wasm32")] {
        // XXX: Do we maybe want to cache this function somewhere at some point?
        let _ = js_sys::eval("debugger;");
        //js_sys::Function::new_no_args("debugger;").call0(&wasm_bindgen::prelude::JsValue::UNDEFINED);
        return;
    }

    // For stable, signals tend to work.  Probably.  Maybe.  Although I recall the best signals (e.g. continuable after
    // a debugger attaches).  `std::intrinsics::breakpoint` is also an option, but it's perma-unstable (nightly only)
    // and adding the required feature to lib.rs for nightly only would require:
    // ```
    // #![rustc::attr(nightly, feature(core_intrinsics))]
    // ```
    // Which is *also* unstable, and thus nightly only, defeating the whole point.  Lame!  Other signals possibly worth
    // using include SIGILL, SIGSTOP, or SIGSEGV, depending on what exact debugger behavior... although SIGTRAP seems
    // like the "correct" signal.  https://en.wikipedia.org/wiki/Signal_(IPC)
    #[cfg(unix)] {
        if state() == State::Attached {
            #[link(name = "c")] extern "C" { fn raise(signum: i32) -> i32; }
            const SIGTRAP : i32 = 5;
            unsafe { raise(SIGTRAP); }
        }
        return;
    }
}

/// Wait for a debugger to be attached to the current process.
/// Will return an `Err("...")` if the debugger state is unknown, or waiting for the debugger times out.
/// 
/// # Platforms
/// 
/// | Platform  | State | Notes |
/// | --------- | ----- | ----- |
/// | Windows   | OK    |       |
/// | Android   | OK    |       |
/// | Linux     | OK    | See `state()` for known bugs, signal type might be wrong/suboptimal for debuggers.
/// | FreeBSD   | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | NetBSD    | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | OS X      | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | iOS       | ???   | Untested, signal type might be wrong/suboptimal for debuggers
/// | WASM      | OK    |       |
/// 
/// # Examples
/// 
/// ```
/// use std::time::Duration;
/// use bugsalot::debugger;
/// 
/// // Wait indefinitely for a debugger to attach.
/// # if false {
/// debugger::wait_until_attached(None).expect("state() not implemented on this platform");
/// # }
/// 
/// // Wait up to a timeout for a debugger to attach.
/// if debugger::wait_until_attached(Duration::from_millis(1)).is_ok() {
///     println!("Debugger attached OK!");
/// }
/// 
/// // Timeout can be an Optional duration as well.
/// match debugger::wait_until_attached(Some(Duration::from_millis(1))) {
///     Ok(()) => println!("Debugger attached OK!"),
///     Err(m) => println!("Debugger didn't attach: {}", m),
/// }
/// ```
pub fn wait_until_attached<T: Into<Option<std::time::Duration>>> (timeout: T) -> Result<(), &'static str> {
    let timeout = timeout.into().map(|dur| std::time::Instant::now() + dur);
    loop {
        match state() {
            State::Attached => return Ok(()),
            State::Unknown  => return Err("Debugger state is unknown, cannot wait_until_attached"),
            State::Detatched => {
                if let Some(timeout) = timeout {
                    if std::time::Instant::now() >= timeout {
                        return Err("wait_until_attached timed out");
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(16)); // Poll at 60hz
            }
        }
    }
}

/// | Function                  | Description |
/// | ------------------------- | ----------- |
/// | `break_or_dump`           | Breakpoint if attached, attempt to create a crash/memory dump otherwise.  Enum to indicate if execution should continue if possible or not?
/// | `attach`                  | Request a debugger be attached to the current process.
/// | `detatch`                 | Requests the attached debugger, if any, detatch and stop debugging us.
/// | `attach_to(process)`      | Request a debugger be attached to another process, possibly e.g. a child process.
/// | `reattach_to(process)`    | Request an attached debugger reattach to a different process (example use case: `cargo run ...`?)
mod possible_future_apis {}
