use require_lifetimes::require_lifetimes;

#[require_lifetimes{compile_error!("The swap function did not have appropriate lifetimes!");}]
fn swap(a: &i32, b: &i32) -> (&i32, &i32) {
    (b, a)
}

fn main() {
    let x = 3;
    let y = 4;
    assert_eq!(swap(&x, &y), (&y, &x));
}
