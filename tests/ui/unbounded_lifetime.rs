#![allow(clippy::needless_lifetimes)]
#![warn(clippy::unbounded_lifetime)]

struct HoldsLt<'a>(&'a str);

#[expect(clippy::unbounded_lifetime)]
fn returns_unbounded<'a>() -> &'a str {
    "Hello"
}

fn returns_bounded<'a>(val: &'a str) -> &'a str {
    val
}

fn returns_bounded_with_ty<'a>(val: HoldsLt<'a>) -> &'a str {
    val.0
}

struct Type;
impl Type {
    fn test_fn_method_generic_layout_asd<'a, T>(arg: &'a T) -> &'a T {
        arg
    }
}

fn main() {}
