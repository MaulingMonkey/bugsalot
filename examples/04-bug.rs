fn main() {
    let range = 0..10;
    for i in range.clone() {
        if i == 5 {
            bugsalot::bug!("5?  Do we look like we allow 5 in {:?}?", &range);
        }
    }
}
