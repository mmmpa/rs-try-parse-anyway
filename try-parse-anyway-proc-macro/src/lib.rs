#![allow(warnings)]

use proc_macro;
use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote};
use serde::{Deserialize, Serialize};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[derive(Clone)]
struct Field<'a> {
    key: String,
    class: String,
    ty: &'a Type,
}

#[proc_macro_derive(TryParseAnything)]
pub fn try_parse_anyway(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut fields: Vec<Field> = vec![];

    match &input.data {
        Data::Struct(data) => {
            match &data.fields {
                Fields::Named(fields) => {
                    fields.named.iter().for_each(|field| {
                        let name = &field.ident.clone().unwrap();
                    });
                }
                Fields::Unnamed(_) => {}
                Fields::Unit => {}
            }

            data.fields.iter().for_each(|field| match &field.ty {
                Type::Path(x) => {
                    fields.push(Field {
                        key: field.ident.as_ref().unwrap().to_string(),
                        class: "".to_string(),
                        ty: &field.ty,
                    });
                }
                _ => panic!("only for simple fields"),
            });
        }
        _ => panic!("only for struct"),
    }

    let base = input.ident.to_string();
    let struct_name = format_ident!("{}", base);
    let value_name = format_ident!("{}RetrievalValue", base);

    let mut keys = vec![];
    let mut keys_str = vec![];
    let mut tyss = vec![];

    let value_fields_raw = fields.iter().for_each(|Field { key, class, ty }| {
        keys.push(format_ident!("{}", key));
        keys_str.push(key);
        tyss.push(ty);
    });

    let gen = quote! {
        impl #struct_name {
            pub fn try_parse_anything(data: &[u8]) -> Result<Self, TryParseAnywayError<#struct_name>> {
                #[derive(Deserialize, Default)]
                #[serde(default)]
                struct PartialValue {
                    #(pub #keys: Value),*
                }

                let mut errors = HashMap::new();
                let v: PartialValue = match serde_json::from_slice(data) {
                    Ok(x) => x,
                    Err(e) => {
                        errors.insert("_", TryParseAnywayErrorItem {
                            value: Value::Null,
                            error: e.to_string(),
                        });
                        return Err(TryParseAnywayError {
                        partial_retrieved: None,
                        errors
                    });
                    },
                };

                #(
                    let #keys: #tyss = match serde_json::from_value(v.#keys.clone()) {
                        Ok(x) => x,
                        Err(e) => {
                            errors.insert(#keys_str, TryParseAnywayErrorItem {
                                value: v.#keys,
                                error: e.to_string(),
                            });
                            Default::default()
                        },
                    };
                )*

                let created = Self {
                    #(#keys),*
                };

                if errors.is_empty() {
                    Ok(created)
                } else {
                    Err(TryParseAnywayError {
                        partial_retrieved: Some(created),
                        errors
                    })
                }
            }
        }
    };

    // println!("{}", gen);

    gen.into()
}
