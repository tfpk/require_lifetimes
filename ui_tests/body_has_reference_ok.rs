use require_lifetimes::require_lifetimes;

#[require_lifetimes]
fn test() {
    let x: i32 = 2;
    let x_ref: &i32 = &x;
    let _ = x_ref;
}

fn main() {
    test();
}
