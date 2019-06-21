use bugsalot::*;

fn main() {
    loop {
        let a : Result<i32, &'static str> = Ok(42);
        let b : Result<i32, &'static str> = Err("Some error");
        let _a = expect!(a, "Unable to do something or other", return);
        let _b = expect!(b, "Unable to do something or other", return); // Debugger will break here
    }
}
