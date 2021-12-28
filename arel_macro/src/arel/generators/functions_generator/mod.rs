pub mod accessor;
pub mod validator;

use expansion::helpers::{self, DeriveInputHelper};
#[allow(unused_imports)]
use syn::{AttributeArgs, spanned::Spanned};

// pub fn generate_struct_functions_define(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
//     let fields = derive_input_helper.get_fields()?;
//
//     // let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
//     // let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();
//
//     let mut final_token_stream = proc_macro2::TokenStream::new();
//
//     // generate table_column_name_functions
//     // {
//     //     let column_names_token_stream = fields.iter().map(|f| {
//     //         if let Some(origin_ident) = &f.ident {
//     //             let fn_name = &syn::Ident::new(&format!("{}_table_column_name", origin_ident.to_string().trim_start_matches("r#")), f.span());
//     //             let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
//     //             if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), None)? {
//     //                 Ok(quote::quote! {
//     //                     pub fn #fn_name() -> &'static str {
//     //                         stringify!(#rename_ident)
//     //                     }
//     //                 })
//     //             }  else {
//     //                 Ok(quote::quote! {
//     //                     pub fn #fn_name() -> &'static str {
//     //                         stringify!(#origin_ident)
//     //                     }
//     //                 })
//     //             }
//     //         } else {
//     //             Ok(quote::quote! {})
//     //         }
//     //     }).collect::<syn::Result<proc_macro2::TokenStream>>()?;
//     //     final_token_stream.extend(column_names_token_stream);
//     // }
//     Ok(final_token_stream)
// }

pub fn generate_struct_impl_arel_functions_define(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    let fields = derive_input_helper.get_fields()?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let arg_allow_attrs = vec!["table_name", "primary_key", "locking_column"];
    // table_name
    {
        if let Some(ident) = helpers::get_macro_nested_attr_value_ident(args.iter().collect(), "table_name", None, Some(arg_allow_attrs.clone()))? {
            // if let Some(table_name_ident) = get_struct_attr(args, "table_name")? {
            let token_stream = quote::quote! {
                fn table_name() -> String {
                    stringify!(#ident).to_string()
                }
            };
            final_token_stream.extend(token_stream);
        }
    }
    // primary_key
    {
        if let Some(ident) = helpers::get_macro_nested_attr_value_ident(args.iter().collect(), "primary_key", None, Some(arg_allow_attrs.clone()))? {
            // if let Some(table_name_ident) = get_struct_attr(args, "table_name")? {
            let token_stream = quote::quote! {
                fn primary_key() -> &'static str {
                    stringify!(#ident)
                }
            };
            final_token_stream.extend(token_stream);
        }
    }
    // locking_column
    // fn set_locking_column_value(_locking_version: i32)
    {
        if let Some(ident) = helpers::get_macro_nested_attr_value_ident(args.iter().collect(), "locking_column", None, Some(arg_allow_attrs.clone()))? {
            // if let Some(table_name_ident) = get_struct_attr(args, "table_name")? {
            let token_stream = quote::quote! {
                fn locking_column() -> std::option::Option<&'static str> {
                    std::option::Option::Some(stringify!(#ident))
                }
            };
            final_token_stream.extend(token_stream);
            // fn set_locking_column_attr_value(_locking_version: i32)
            let token_stream = quote::quote! {
                fn set_locking_column_attr_value(&mut self, locking_version: i32) -> arel::anyhow::Result<()> {
                    self.#ident = std::option::Option::Some(locking_version.try_into()?);
                    Ok(())
                }
            };
            final_token_stream.extend(token_stream);
            // get_persisted_locking_column_attr_value(&self)
            let token_stream = quote::quote! {
                fn get_persisted_locking_column_attr_value(&self) -> arel::anyhow::Result<std::option::Option<i32>> {
                    if let std::option::Option::Some(persisted_row_record) = &self.persisted_row_record {
                        if let std::option::Option::Some(locking_version) = persisted_row_record.#ident {
                            return std::result::Result::Ok(std::option::Option::Some(locking_version.try_into()?))
                        }
                    }
                    return std::result::Result::Ok(std::option::Option::None)
                }
            };
            final_token_stream.extend(token_stream);
        }
    }
    // table_column_names
    {
        let mut idents: Vec<_> = vec![];
        for f in fields.iter() {
            if let Some(ident) = &f.ident {
                let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
                if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), None)? {
                    idents.push(rename_ident);
                } else {
                    idents.push(ident.clone());
                }
            }
        }
        final_token_stream.extend(quote::quote! {
            fn table_column_names() -> Vec<&'static str> {
               vec![
                    #(stringify!(#idents),)*
                ]
            }
        })
    }
    // attr_names
    {
        final_token_stream.extend(quote::quote! {
             fn attr_names() -> Vec<&'static str> {
                vec![
                    #(stringify!(#idents),)*
                ]
            }
        })
    }
    // attr_name_to_table_column_name
    {
        let mut match_tokens = vec![];
        for f in fields.iter() {
            if let Some(ident) = &f.ident {
                let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
                if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), None)? {
                    match_tokens.push(quote::quote! {
                        stringify!(#ident) => std::result::Result::Ok(stringify!(#rename_ident))
                    });
                } else {
                    match_tokens.push(quote::quote! {
                        stringify!(#ident) => std::result::Result::Ok(stringify!(#ident))
                    });
                }
            }
        }
        final_token_stream.extend(quote::quote! {
             fn attr_name_to_table_column_name<'a>(attr_name: &'a str) -> anyhow::Result<&'a str> {
                match attr_name {
                    #(#match_tokens,)*
                    _ => std::result::Result::Err(anyhow::anyhow!("attr_name_to_table_column_name: {} Not Found", attr_name))

                }
            }
        })
    }
    // fn table_column_name_to_attr_name<'a>(table_column_name: &'a str) -> anyhow::Result<&'a str>;
    {
        let mut match_tokens = vec![];
        for f in fields.iter() {
            if let Some(ident) = &f.ident {
                let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
                if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), None)? {
                    match_tokens.push(quote::quote! {
                        stringify!(#rename_ident) => std::result::Result::Ok(stringify!(#ident))
                    });
                } else {
                    match_tokens.push(quote::quote! {
                        stringify!(#ident) => std::result::Result::Ok(stringify!(#ident))
                    });
                }
            }
        }
        final_token_stream.extend(quote::quote! {
             fn table_column_name_to_attr_name<'a>(table_column_name: &'a str) -> anyhow::Result<&'a str> {
                match table_column_name {
                    #(#match_tokens,)*
                    _ => std::result::Result::Err(anyhow::anyhow!("table_column_name_to_attr_name: {} Not Found", table_column_name))

                }
            }
        })
    }
    // attr_json
    {
        final_token_stream.extend(quote::quote! {
            fn attr_json(&self, attr: &str) -> std::option::Option<arel::serde_json::Value> {
                match attr {
                    #(
                        stringify!(#idents) => {
                            if let std::option::Option::Some(value) = self.#idents() {
                                return std::option::Option::Some(arel::serde_json::json!(value));
                            }
                        },
                    )*
                    _ => (),
                }
                std::option::Option::None
            }
        })
    }
    // persisted_attr_json
    {
        final_token_stream.extend(quote::quote! {
            fn persisted_attr_json(&self, attr: &str) -> std::option::Option<arel::serde_json::Value> {
                if let std::option::Option::Some(persisted_row_record) = self.persisted_row_record() {
                    match attr {
                        #(
                            stringify!(#idents) => {
                                if let std::option::Option::Some(value) = &persisted_row_record.#idents {
                                    return std::option::Option::Some(arel::serde_json::json!(value));
                                }
                            },
                        )*
                        _ => (),
                    }
                }
                std::option::Option::None
            }
        })
    }

    Ok(quote::quote! {
        #final_token_stream
    })
}