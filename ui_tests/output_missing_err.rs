use require_lifetimes::require_lifetimes;

#[require_lifetimes]
fn identity<'a, 'b>(a: &'a i32, _b: &'b i32) -> &i32 {
    a
}

fn main() {}
