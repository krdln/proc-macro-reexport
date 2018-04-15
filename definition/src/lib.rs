#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn foo(_input: TokenStream) -> TokenStream {
    unimplemented!()
    // _input
}
