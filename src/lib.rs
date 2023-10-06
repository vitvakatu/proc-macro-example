extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Data};
use quote::quote;

#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

    let struct_name = &macro_input.ident;

    let field_count = match &macro_input.data {
        Data::Struct(data) => data.fields.iter().count(),
        _ => panic!("Count derive macro only works on structs"),
    };

    let expanded = quote! {
        impl #struct_name {
            pub fn field_count() -> usize {
                #field_count
            }
        }
    };

    expanded.into()
}


#[cfg(test)]
mod tests {
    use trybuild::TestCases;

    #[test]
    fn test_count_macro() {
        let t = TestCases::new();

        // Test cases where the macro should succeed.
        t.pass("tests/01_basic.rs");
        t.pass("tests/02_multiple_fields.rs");
    }
}
