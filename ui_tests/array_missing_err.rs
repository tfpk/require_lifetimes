use require_lifetimes::require_lifetimes;

#[require_lifetimes]
fn identity<'a>(a: &'a [&i32; 5]) -> &'a [&i32; 5] {
    a
}

fn main() {}
