#![feature(proc_macro, proc_macro_non_items)]

// #[macro_use]
extern crate reexport;

use reexport::foo;

fn main() {
    foo!(println!("hello, world!"))
}
