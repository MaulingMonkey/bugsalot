use bugsalot::*;

fn main() {
    let a : Option<i32> = Some(42);
    let b : Option<i32> = None;
    let _a = unwrap!(a);
    let _b = unwrap!(b); // Debugger will break here
}
