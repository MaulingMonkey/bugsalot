pub type BOOL = i32;

#[link(name = "kernel32")]
extern "system" {
    pub fn DebugBreak();
    pub fn IsDebuggerPresent() -> BOOL;
    pub fn OutputDebugStringA(lpOutputString: *const u8);
}
