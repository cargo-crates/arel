use expansion::helpers::{self, DeriveInputHelper};
#[allow(unused_imports)]
use syn::{AttributeArgs, spanned::Spanned};

pub fn generate_belongs_to_associations(_derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    if let Some(belongs_to_args_vec) = helpers::get_namespace_nested_metas_vec(args.iter().collect(), vec!["belongs_to"])? {
        for belongs_to_args in belongs_to_args_vec.into_iter() {
            if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = belongs_to_args.get(0).unwrap() {
                let association_name = name.value();
                let association_ident = syn::Ident::new(&association_name, name.span());
                if let Some(belongs_to_struct_ident) = helpers::get_macro_nested_attr_value_ident(belongs_to_args.clone(), "struct", None, None)? {
                    let belongs_to_struct_name = belongs_to_struct_ident.to_string();
                    let foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&belongs_to_struct_name));
                    let mut foreign_key_ident = syn::Ident::new(&foreign_key, belongs_to_struct_ident.span());
                    if let Some(custom_foreign_key_ident) = helpers::get_macro_nested_attr_value_ident(belongs_to_args.clone(), "foreign_key", None, None)? {
                        // foreign_key = custom_foreign_key_ident.to_string();
                        foreign_key_ident = custom_foreign_key_ident;
                    }
                    final_token_stream.extend(quote::quote! {
                        pub fn #association_ident(&self) -> arel::anyhow::Result<arel::table::Table<#belongs_to_struct_ident>> {
                            let attr_foreign_key = Self::table_column_name_to_attr_name(stringify!(#foreign_key_ident))?;
                            if let Some(attr_foreign_key_json) = self.persisted_attr_json(attr_foreign_key) {
                                let mut map = arel::serde_json::Map::new();
                                map.insert(#belongs_to_struct_ident::primary_key().to_string(), attr_foreign_key_json);
                                let mut query = #belongs_to_struct_ident::query();
                                query.r#where(arel::serde_json::Value::Object(map));
                                std::result::Result::Ok(query)
                            } else {
                                std::result::Result::Err(arel::anyhow::anyhow!("Model foreign_key attr {} is Blank", attr_foreign_key))
                            }
                        }
                    });
                    let join_association_ident = syn::Ident::new(&format!("{}_join_string", association_ident.to_string()), association_ident.span());
                    final_token_stream.extend(quote::quote! {
                        pub fn #join_association_ident() -> String {
                            let assocation_table_name = #belongs_to_struct_ident::table_name();
                            let assocation_primary_key = #belongs_to_struct_ident::primary_key();
                            let self_table_name = Self::table_name();
                            let self_foreign_key = stringify!(#foreign_key_ident);
                            // check foreign_key exists
                            let self_table_columns = Self::table_column_names();
                                if !self_table_columns.contains(&self_foreign_key) {
                                    panic!("belongs_to foreign_key({}) Not In Table {} Columns: {:?}", self_foreign_key, self_table_name, self_table_columns);
                                }


                            format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, assocation_table_name, assocation_primary_key, self_table_name, self_foreign_key)
                        }
                    });
                } else {
                    return Err(syn::Error::new_spanned(name, "loss attr: struct".to_string()))
                }
            }
        }
    }
    Ok(final_token_stream)
}