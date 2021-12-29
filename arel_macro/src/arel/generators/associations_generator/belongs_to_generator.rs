use expansion::helpers::{self, DeriveInputHelper};
#[allow(unused_imports)]
use syn::{AttributeArgs, spanned::Spanned};

pub fn get_belongs_to_args_vec(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<Option<Vec<Vec<syn::NestedMeta>>>> {
    let mut total_vec = vec![];
    let metas = helpers::parse_attrs_to_metas(&derive_input_helper.value().attrs)?;
    if let Some(mut vec) = helpers::get_namespace_nested_metas_vec_from_metas(metas.iter().collect::<Vec<_>>(), vec!["belongs_to"])? {
        total_vec.append(&mut vec)
    }
    if let Some(mut vec) = helpers::get_namespace_nested_metas_vec_from_nested_metas(args.iter().collect(), vec!["belongs_to"])? {
        total_vec.append(&mut vec)
    }
    if total_vec.len() > 0 {
        Ok(Some(total_vec.into_iter().map(|i| i.into_iter().map(|i| i.clone()).collect::<Vec<_>>()).collect::<Vec<_>>()))
    } else {
        Ok(None)
    }
}

pub fn handle_association_attributes(belongs_to_args: Vec<&syn::NestedMeta>, _derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident)>> {
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
            return Ok(Some((association_ident, belongs_to_struct_ident, foreign_key_ident)))
        } else {
            return Err(syn::Error::new_spanned(name, "loss attr: struct".to_string()))
        }
    }
    Ok(None)
}

pub fn generate_belongs_to_associations(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    if let Some(belongs_to_args_vec) = get_belongs_to_args_vec(derive_input_helper, args)? {
        for belongs_to_args in belongs_to_args_vec.into_iter() {
            if let Some((association_ident, belongs_to_struct_ident, foreign_key_ident)) = handle_association_attributes(belongs_to_args.iter().collect(), derive_input_helper, args)? {
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
            }
        }
    }
    Ok(final_token_stream)
}