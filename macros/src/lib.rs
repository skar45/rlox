use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn rlox_error(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);

    if let syn::Data::Struct(ref mut struct_data) = input.data {
        if let Fields::Named(ref mut fields) = struct_data.fields {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! { pub line: usize })
                    .unwrap(),
            );
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! { pub column: usize })
                    .unwrap(),
            );
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! { pub msg: String })
                    .unwrap(),
            );
        }
    }

    quote!(#input).into()
}

#[cfg(test)]
mod tests {}
