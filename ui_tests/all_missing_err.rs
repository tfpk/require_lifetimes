use require_lifetimes::require_lifetimes;

#[require_lifetimes]
fn identity(a: &i32, _b: &i32) -> &i32 {
    a
}

fn main() {}
