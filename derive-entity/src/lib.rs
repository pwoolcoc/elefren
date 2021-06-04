const EXTRAS_FIELD: &str = "elefren_extra";

#[proc_macro_derive(Entity, attributes(entity))]
pub fn entity(input: TokenStream) -> TokenStream {
    let DeriveInput { attrs, vis, ident, data, .. } = parse_macro_input!(input as DeriveInput);

    let tokens: Vec<NestedMeta> = if let Some(attr) = attrs.into_iter().find(|attr| attr.path.is_ident("entity")) {
        match attr.parse_meta().expect("Couldn't parse meta") {
            Meta::List(meta) => Vec::from_iter(meta.nested),
            _ => vec![],
        }
    } else {
        vec![]
    };

    let field: proc_macro2::Ident = tokens.iter().find_map(|token| {
        match token {
            NestedMeta::Meta(Meta::NameValue(name_value)) => {
                if name_value.path.is_ident("extras_field") {
                    let value = match name_value.lit {
                        syn::Lit::Str(ref lit_str) => match lit_str.parse() {
                            Ok(ident) => Some(ident),
                            _ => None,
                        },
                        _ => None,
                    };
                    value
                } else {
                    None
                }
            },
            _ => None,
        }
    }).unwrap_or_else(|| proc_macro2::Ident::new(EXTRAS_FIELD, proc_macro2::Span::call_site()));

    if let Data::Struct(syn::DataStruct { .. }) = data {
        return quote! {
            impl #ident {
                #vis fn contains_key(&self, key: &str) -> bool {
                    self.#field.contains_key(key)
                }
                #vis fn keys<'this>(&'this self) -> ::std::collections::hash_map::Keys<'this, String, ::serde_json::Value> {
                    self.#field.keys()
                }
                #vis fn get(&self, key: &str) -> Option<&Value> {
                    self.#field.get(key)
                }
            }
        }.into();
    }

    panic!("Can't derive Entity on an enum or union");
}

use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, NestedMeta, Meta, Data};
use std::iter::FromIterator;
