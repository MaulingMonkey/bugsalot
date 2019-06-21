use bugsalot::*;

fn main() {
    let a : Option<i32> = Some(42);
    let b : Option<i32> = None;
    let _a = expect!(a, "Unable to do something or other", return);
    let _b = expect!(b, "Unable to do something or other", return); // Debugger will break here
}
