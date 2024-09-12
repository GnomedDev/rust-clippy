#![warn(clippy::unneccessary_literal_bound)]

struct Struct<'a> {
    not_literal: &'a str,
}

impl Struct<'_> {
    fn returns_lit(&self) -> &str {
        "Hello"
    }

    fn conditionally_returns_lit(&self, cond: bool) -> &str {
        if cond { "Literal" } else { self.not_literal }
    }

    fn returns_literals_explicit(&self, cond: bool) -> &str {
        if cond {
            return "Literal";
        }

        "also a literal"
    }
}

trait ReturnsStr<'a> {
    fn must_return_bounded(self) -> &'a str;
    fn because_of_other(self) -> &'a str;
}

impl<'a> ReturnsStr<'a> for Struct<'a> {
    fn must_return_bounded(self) -> &'a str {
        "Literal"
    }

    fn because_of_other(self) -> &'a str {
        self.not_literal
    }
}

fn check_for_str_ret_please(other: &str) -> &str {
    "Hello"
}

fn main() {}
