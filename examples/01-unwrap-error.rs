use bugsalot::*;

fn main() {
    let a : Result<i32, &'static str> = Ok(42);
    let b : Result<i32, &'static str> = Err("Some error");
    let _a = unwrap!(a);
    let _b = unwrap!(b); // Debugger will break here
}
