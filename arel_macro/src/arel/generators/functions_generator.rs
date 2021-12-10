use expansion::helpers::{self, DeriveInputHelper};
use syn::{AttributeArgs, spanned::Spanned};

pub fn generate_struct_functions_define(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fileds = derive_input_helper.get_fields()?;

    let idents: Vec<_> = fileds.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fileds.iter().map(|f| &f.ty).collect();

    let mut final_token_stream = proc_macro2::TokenStream::new();

    // generate table_column_name_functions
    {
        let column_names_token_stream = fileds.iter().map(|f| {
            if let Some(origin_ident) = &f.ident {
                let fn_name = &syn::Ident::new(&format!("{}_table_column_name", origin_ident.to_string()), f.span());
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

    // generate table_column_setter_functions
    // quote::quote! {
    //     #(pub fn #idents(&mut self, #idents: #types) -> &mut Self {
    //         self.#idents = std::option::Option::Some(#idents);
    //         self
    //     })*
    // },
    {
        let mut setters_token_stream = proc_macro2::TokenStream::new();
        for (ident, r#type) in idents.iter().zip(types.iter()) {
            let token_stream_piece;
            if let Some(_) = helpers::get_type_inner_type_ident(r#type, "Vec") {
                token_stream_piece = quote::quote! {
                    fn #ident(&mut self, #ident: #r#type) -> &mut Self {
                        self.#ident = #ident;
                        self
                    }
                };
            }  else {
                // T -> T, Option<T> -> T
                let r#type = if let Some(inner_type) = helpers::get_type_inner_type_ident(r#type, "Option") { inner_type } else { r#type };
                token_stream_piece = quote::quote! {
                    fn #ident(&mut self, #ident: #r#type) -> &mut Self {
                        self.#ident = std::option::Option::Some(#ident);
                        self
                    }
                };
            }
            setters_token_stream.extend(token_stream_piece);
        }
        final_token_stream.extend(setters_token_stream);
    }

    // generate struct validations_function
    {
        let segments: Vec<_> = fileds.iter().filter(|f| {
              helpers::get_type_inner_type_ident(&f.ty, "Option").is_none() && helpers::get_type_inner_type_ident(&f.ty, "Vec").is_none()
            }).map(|f| {
                let ident = &f.ident;
                quote::quote! {
                    if self.#ident.is_none() {
                        return Err(anyhow::anyhow!("{} Not Allow None", stringify!(#ident)));
                    }
                }
            }).collect();
        let validations_token_stream = quote::quote! {
            pub fn validate(&self) -> anyhow::Result<()> {
                #(#segments)*
                Ok(())
            }
        };
        final_token_stream.extend(validations_token_stream);
    }
    Ok(final_token_stream)
}

pub fn generate_struct_impl_arel_functions_define(_derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();

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

    Ok(quote::quote! {
        #final_token_stream
        // fn primary_key() -> &'static str { "id" }
    })
}