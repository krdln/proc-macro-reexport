#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn foo(input: TokenStream) -> TokenStream {
    // unimplemented!()
    input
}
