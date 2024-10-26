extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{ItemEnum, parse_macro_input, TypePath};
use proc_macro::TokenStream;

/// ignore_field
/// snake
#[proc_macro_derive(Display, attributes(ignore_field, snake))]
pub fn enum_display_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    impl_display(input)
}

fn impl_display(input: ItemEnum) -> TokenStream {
    let name = &input.ident;
    let mut tokens:Vec<proc_macro2::TokenStream> = vec![];
    let mut global_ignore_field = false;
    for attr in &input.attrs {
        if attr.path().is_ident("ignore_field") {
            global_ignore_field = true;
            break;
        }
    }
    for variant in input.variants {
        let mut vars = vec![];
        let mut ignore_field = false;
        for attr in variant.attrs {
            if attr.path().is_ident("ignore_field") {
                ignore_field = true;
                break;
            }
        }
        for (idx,field) in variant.fields.iter().enumerate() {
            if let syn::Type::Path(TypePath{ path,.. }) =  &field.ty {
                for _seg in path.segments.iter() {
                    let data = Ident::new(format!("val{}", idx).as_str(), Span::call_site());
                    vars.push(quote! { #data });
                }
            }
        }
        let ident = variant.ident;
        if vars.len() > 0 {
            let fields = quote! { #(#vars),*};
            if global_ignore_field || ignore_field{
                tokens.push(quote! {
                    Self::#ident(#fields) => write!(f, "{}", stringify!(#ident))
                })
            } else {
                tokens.push(quote! {
                    Self::#ident(#fields) => write!(f, "{}:{:?}", stringify!(#ident), (#fields))
                })
            }
        } else {
            tokens.push(quote! {
                Self::#ident => write!(f, stringify!(#ident))
            });
        }
    }

    let token = quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#tokens),*
                }
            }
        }
    };
    TokenStream::from(token)
}