use require_lifetimes::require_lifetimes;

#[require_lifetimes]
fn identity<'a>(a: &'a i32, _b: &'_ i32) -> &'a i32 {
    a
}

fn main() {}
