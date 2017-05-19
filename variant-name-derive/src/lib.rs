extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{Variant, VariantData, Ident, Body, TyParam};

#[proc_macro_derive(VariantName)]
pub fn variant_name(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let name = &ast.ident;
    let ty_params = &ast.generics.ty_params;
    let gen = match ast.body {
        Body::Enum(ref variants) => impl_variant_name(name, variants, ty_params),
        Body::Struct(_) =>
            panic!("#[derive(VariantName)] is only defined for enums, not structs"),
    };
    gen.parse().unwrap()
}

fn impl_variant_name(name: &Ident, variants: &[Variant], ty_params: &[TyParam]) -> quote::Tokens {
    let match_variant = match_variant(&name, variants);
    let types = get_type_params(ty_params);
    let type_params = quote! { <#(#types)*> };
    quote! {
        impl#type_params VariantName for #name#type_params {
            fn variant_name(self: &#name#type_params) -> &str {
                match *self {
                    #(#match_variant)*
                }
            }
        }
    }
}

fn get_type_params(ty_params: &[TyParam]) -> Vec<quote::Tokens> {
    ty_params.iter().map(|ty_param| {
        let type_ident = &ty_param.ident;
        quote! {
            #type_ident,
        }
    }).collect()
}

fn match_variant(name: &Ident, variants: &[Variant]) -> Vec<quote::Tokens> {
    let mut result = Vec::new();

    for variant in variants.iter() {
        let id = &variant.ident;
        let new = match variant.data {
            VariantData::Unit => quote! {
                    #name::#id => stringify!(#id),
                },
            VariantData::Tuple(_) => quote! {
                    #name::#id(..) => stringify!(#id),
                },
            VariantData::Struct(_) => quote! {
                    #name::#id { .. } => stringify!(#id),
                },
        };
        result.push(new);
    }

    result
}
