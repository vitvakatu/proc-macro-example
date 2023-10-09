extern crate proc_macro;
use proc_macro::TokenStream;

use syn::{parse_macro_input, DeriveInput, Data, Fields };
use quote::quote;

#[proc_macro_derive(Count)]
pub fn count(input: TokenStream) -> TokenStream {
    let macro_input = parse_macro_input!(input as DeriveInput);

    let struct_or_enum_name = &macro_input.ident;

    let count = match &macro_input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => fields_named.named.len(),
            Fields::Unnamed(fields_unnamed) => fields_unnamed.unnamed.len(),
            Fields::Unit => 0,
        },
        Data::Enum(data_enum) => data_enum.variants.len(),
        _ => panic!("Count derive macro only works on structs and enums"),
    };

    let expanded = quote! {
        impl #struct_or_enum_name {
            pub fn field_count() -> usize {
                #count
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
