extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

/// Example of user-defined [derive mode macro][1]
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#derive-mode-macros
#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

    // Your code here

    quote!(
        // Your code here
    ).into()
}
