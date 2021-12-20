pub mod accessor;
pub mod validator;

use expansion::helpers::{self, DeriveInputHelper};
use syn::{AttributeArgs, spanned::Spanned};

pub fn generate_struct_functions_define(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fields = derive_input_helper.get_fields()?;

    // let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_token_stream = proc_macro2::TokenStream::new();

    // generate table_column_name_functions
    {
        let column_names_token_stream = fields.iter().map(|f| {
            if let Some(origin_ident) = &f.ident {
                let fn_name = &syn::Ident::new(&format!("{}_table_column_name", origin_ident.to_string().trim_start_matches("r#")), f.span());
                let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
                if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), Some(vec!["table_column_name"]))? {
                    Ok(quote::quote! {
                        pub fn #fn_name() -> &'static str {
                            stringify!(#rename_ident)
                        }
                    })
                }  else {
                    Ok(quote::quote! {
                        pub fn #fn_name() -> &'static str {
                            stringify!(#origin_ident)
                        }
                    })
                }
            } else {
                Ok(quote::quote! {})
            }
        }).collect::<syn::Result<proc_macro2::TokenStream>>()?;
        final_token_stream.extend(column_names_token_stream);
    }
    Ok(final_token_stream)
}

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
    {
        if let Some(ident) = helpers::get_macro_nested_attr_value_ident(args.iter().collect(), "locking_column", None, Some(arg_allow_attrs.clone()))? {
            // if let Some(table_name_ident) = get_struct_attr(args, "table_name")? {
            let token_stream = quote::quote! {
                fn locking_column() -> &'static str {
                    stringify!(#ident)
                }
            };
            final_token_stream.extend(token_stream);
        }
    }
    // table_column_names
    {
        let get_table_column_names: Vec<_> = fields.iter().filter_map(|f| {
            if let Some(ident) = &f.ident {
                let get_column_name = format!("{}_table_column_name", ident.to_string().trim_start_matches("r#"));
                let get_column_name_ident = &syn::Ident::new(&get_column_name, f.ident.span());
                Some(quote::quote! {
                    Self::#get_column_name_ident(),
                })
            } else {
                None
            }
        }).collect();
        final_token_stream.extend(quote::quote! {
            fn table_column_names() -> Vec<&'static str> {
                vec![
                    #(#get_table_column_names)*
                ]
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
                if let Some(persisted_row_record) = self.persisted_row_record() {
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