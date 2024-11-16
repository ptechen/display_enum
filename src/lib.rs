extern crate proc_macro;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{ItemEnum, parse_macro_input, TypePath, TypeTuple};
use proc_macro::TokenStream;
use syn::spanned::Spanned;

/// ignore_field
#[proc_macro_derive(Display, attributes(ignore_field, to_vec))]
pub fn enum_display_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);
    impl_display(input)
}

#[derive(Default)]
struct Vars {
    vars: Vec<proc_macro2::TokenStream>,
    enum_type: EnumType,
}

#[derive(Default, PartialEq)]
enum EnumType {
    #[default]
    None,
    Tuple,
    Struct,
}

fn impl_display(input: ItemEnum) -> TokenStream {
    let name = &input.ident;
    let mut tokens:Vec<proc_macro2::TokenStream> = vec![];
    let mut global_ignore_field = false;
    for attr in &input.attrs {
        if attr.path().is_ident("ignore_field") {
            global_ignore_field = true;
        }
    }
    for variant in input.variants {
        let mut vars = Vars::default();
        let mut ignore_field = false;
        for attr in variant.attrs {
            if attr.path().is_ident("ignore_field") {
                ignore_field = true;
                break;
            }
        }
        for (idx,field) in variant.fields.iter().enumerate() {
            if let syn::Type::Path(TypePath{ path,.. }) =  &field.ty {
                if let Some(ident) = &field.ident {
                    vars.enum_type = EnumType::Struct;
                    for _seg in path.segments.iter() {
                        eprintln!("111:{}", field.ident.clone().unwrap().to_string());
                        vars.vars.push(quote! { #ident });
                    }
                } else {
                    for _seg in path.segments.iter() {
                        eprintln!("111:");
                        vars.enum_type = EnumType::Tuple;
                        let data = Ident::new(format!("val{}", idx).as_str(), Span::call_site());
                        vars.vars.push(quote! { #data });
                    }
                }
            }
        }
        let ident = variant.ident;
        if vars.vars.len() > 0 {
            let fields = &vars.vars;
            let fields = quote! { #(#fields),*};
            if global_ignore_field || ignore_field{
                if vars.enum_type == EnumType::Tuple {
                    tokens.push(quote! {
                        Self::#ident(#fields) => write!(f, "{}", stringify!(#ident))
                    })
                } else if vars.enum_type == EnumType::Struct {
                    tokens.push(quote! {
                        Self::#ident{#fields} => write!(f, "{}", stringify!(#ident))
                    })
                }
            } else {
                if vars.enum_type == EnumType::Tuple {
                    tokens.push(quote! {
                        Self::#ident(#fields) => write!(f, "{}:{:?}", stringify!(#ident), (#fields))
                    })
                } else if vars.enum_type == EnumType::Struct {
                    tokens.push(quote! {
                        Self::#ident{#fields} => write!(f, "{}:{:?}", stringify!(#ident), (#fields))
                    })
                }
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