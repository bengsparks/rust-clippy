#![warn(clippy::recommend_deriving)]

struct Foo(u64);

impl PartialEq for Foo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

fn main() {
    // test code goes here
}
