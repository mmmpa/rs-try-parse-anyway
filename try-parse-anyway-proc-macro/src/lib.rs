use proc_macro;
use proc_macro::{TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Type};

#[derive(Clone)]
struct Field<'a> {
    key: String,
    ty: &'a Type,
}

#[proc_macro_derive(TryParseAnyway)]
pub fn try_parse_anyway(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut fields: Vec<Field> = vec![];

    match &input.data {
        Data::Struct(data) => {
            data.fields.iter().for_each(|field| match &field.ty {
                Type::Path(_) => {
                    fields.push(Field {
                        key: field.ident.as_ref().unwrap().to_string(),
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

    let mut keys = vec![];
    let mut keys_str = vec![];
    let mut tyss = vec![];

    fields.iter().for_each(|Field { key, ty }| {
        keys.push(format_ident!("{}", key));
        keys_str.push(key);
        tyss.push(ty);
    });

    let gen = quote! {
        impl #struct_name {
            pub fn try_from_slice_anyway(data: &[u8]) -> Result<Self, TryParseAnywayError<#struct_name>> {
                #[derive(Deserialize, Default)]
                #[serde(default)]
                struct PartialValue {
                    #(pub #keys: Value),*
                }

                let mut errors = HashMap::new();
                let v: PartialValue = match serde_json::from_slice(data) {
                    Ok(x) => x,
                    Err(e) => return Err(TryParseAnywayError::Total(e.to_string())),
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

                let retrieved = Self {
                    #(#keys),*
                };

                if errors.is_empty() {
                    Ok(retrieved)
                } else {
                    Err(TryParseAnywayError::Partial {
                        retrieved,
                        errors
                    })
                }
            }

            pub fn try_from_str_anyway(data: &str) -> Result<Self, TryParseAnywayError<#struct_name>> {
                #struct_name::try_from_slice_anyway(data.as_bytes())
            }

            pub fn try_from_value_anyway(data: Value) -> Result<Self, TryParseAnywayError<#struct_name>> {
                #[derive(Deserialize, Default)]
                #[serde(default)]
                struct PartialValue {
                    #(pub #keys: Value),*
                }

                let mut errors = HashMap::new();
                let v: PartialValue = match serde_json::from_value(data) {
                    Ok(x) => x,
                    Err(e) => return Err(TryParseAnywayError::Total(e.to_string())),
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

                let retrieved = Self {
                    #(#keys),*
                };

                if errors.is_empty() {
                    Ok(retrieved)
                } else {
                    Err(TryParseAnywayError::Partial {
                        retrieved,
                        errors
                    })
                }
            }
        }

        impl ParseAnyway<#struct_name> for &[u8] {
            fn parse_anyway(self) ->  Result<#struct_name, TryParseAnywayError<#struct_name>> {
                #struct_name::try_from_slice_anyway(self)
            }
        }

        impl ParseAnyway<#struct_name> for &str {
            fn parse_anyway(self) ->  Result<#struct_name, TryParseAnywayError<#struct_name>> {
                #struct_name::try_from_str_anyway(self)
            }
        }

        impl ParseAnyway<#struct_name> for Value {
            fn parse_anyway(self) ->  Result<#struct_name, TryParseAnywayError<#struct_name>> {
                #struct_name::try_from_value_anyway(self)
            }
        }
   };

    // println!("{}", gen);

    gen.into()
}
