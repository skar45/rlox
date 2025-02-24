use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_attribute]
pub fn rlox_error(attr: TokenStream, item: TokenStream) -> TokenStream {
    let custom_msg = attr.to_string();
    let mut input = parse_macro_input!(item as DeriveInput);

    if let Data::Struct(ref mut struct_data) = input.data {
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

    let struct_name = &input.ident;

    let output =  quote! {
        #input

        impl Error for #struct_name {}

        impl Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if #custom_msg.len() > 0 {
                    write!(f, "{} {}", #custom_msg, &self.msg)
                } else {
                    write!(f, "{}", &self.msg)
                }
            }
        }

        impl ReportError for #struct_name {
            fn get_line(&self) -> usize {
                self.line
            }
            fn get_column(&self) -> usize {
                self.column
            }
            fn get_msg(&self) -> &str {
               &self.msg
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn rlox_error_enum(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as DeriveInput);
    let enum_name = &input.ident;
    let mut variant_impls = Vec::new();

    if let Data::Enum(ref mut enum_data) = input.data {
        for variant in &enum_data.variants {
            if let Fields::Unnamed(fields) = &variant.fields {
                if let Some(field) = fields.unnamed.first() {
                    let variant_name = &variant.ident;
                    let struct_type = &field.ty;

                    let impl_block = quote! {
                        impl From<#struct_type> for #enum_name {
                            fn from(value: #struct_type) -> Self {
                                #enum_name::#variant_name(value)
                            }
                        }
                    };

                    variant_impls.push(impl_block);
                }
            }
        }
    };

    let output = quote! {
        #input

        #(#variant_impls)*
    };
    output.into()
}

#[cfg(test)]
mod tests {}
