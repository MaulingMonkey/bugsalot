pub const SIGTRAP : i32 = 5;

#[link(name = "c")]
extern "C" {
    pub fn raise(signum: i32) -> i32;
}
