#[feature(proc_macro)]

// #[macro_use]
extern crate reexport;

use reexport::foo;

fn main() {
    foo!()
}
