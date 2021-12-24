use expansion::helpers::{self, DeriveInputHelper};
#[allow(unused_imports)]
use syn::{AttributeArgs, spanned::Spanned};

pub fn handle_association_attributes(has_and_belongs_to_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident, syn::Ident, syn::Ident)>> {
    let self_struct_ident = &derive_input_helper.value().ident;
    let self_struct_name = format!("{}", self_struct_ident.to_string());
    let self_struct_ident = &syn::Ident::new(&self_struct_name, derive_input_helper.value().span());

    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_and_belongs_to_args.get(0).unwrap() {
        let association_name = name.value();
        let association_ident = syn::Ident::new(&association_name, name.span());

        if let Some(has_many_struct_ident) = helpers::get_macro_nested_attr_value_ident(has_and_belongs_to_args.clone(), "struct", None, None)? {
            let foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&self_struct_ident.to_string()));
            let mut foreign_key_ident = syn::Ident::new(&foreign_key, has_many_struct_ident.span());
            if let Some(custom_foreign_key_ident) = helpers::get_macro_nested_attr_value_ident(has_and_belongs_to_args.clone(), "foreign_key", None, None)? {
                foreign_key_ident = custom_foreign_key_ident;
            }

            let association_foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&has_many_struct_ident.to_string()));
            let mut association_foreign_key_ident = syn::Ident::new(&association_foreign_key, has_many_struct_ident.span());
            if let Some(custom_association_foreign_key_ident) = helpers::get_macro_nested_attr_value_ident(has_and_belongs_to_args.clone(), "association_foreign_key", None, None)? {
                association_foreign_key_ident = custom_association_foreign_key_ident;
            }

            let mut join_table_vec = vec![
                inflector::string::pluralize::to_plural(&inflector::cases::snakecase::to_snake_case(&self_struct_ident.to_string())),
                inflector::string::pluralize::to_plural(&inflector::cases::snakecase::to_snake_case(&association_ident.to_string())),
            ];
            join_table_vec.sort();
            let join_table = join_table_vec.join("_");
            let mut join_table_ident = syn::Ident::new(&join_table, has_many_struct_ident.span());
            if let Some(custom_join_table_ident) = helpers::get_macro_nested_attr_value_ident(has_and_belongs_to_args.clone(), "join_table", None, None)? {
                join_table_ident = custom_join_table_ident;
            }
            return Ok(Some((association_ident, has_many_struct_ident, foreign_key_ident, association_foreign_key_ident, join_table_ident)))
        } else {
            return Err(syn::Error::new_spanned(name, "loss attr: struct".to_string()))
        }
    }
    Ok(None)
}

pub fn generate_has_and_belongs_to_many_associations(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();

    if let Some(has_and_belongs_to_many_args_vec) = helpers::get_namespace_nested_metas_vec(args.iter().collect(), vec!["has_and_belongs_to_many"])? {
        for has_and_belongs_to_args in has_and_belongs_to_many_args_vec.into_iter() {
            if let Some((association_ident, has_many_struct_ident, foreign_key_ident, association_foreign_key_ident, join_table_ident)) = handle_association_attributes(has_and_belongs_to_args, derive_input_helper, args)? {
                final_token_stream.extend(quote::quote! {
                    pub fn #association_ident(&self) -> arel::anyhow::Result<arel::table::Table<#has_many_struct_ident>> {
                        let assocation_table_name = #has_many_struct_ident::table_name();
                        let assocation_primary_key = #has_many_struct_ident::primary_key();

                        let join_table_name = stringify!(#join_table_ident);

                        let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
                        if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
                            let mut query = #has_many_struct_ident::query();
                            let full_join_string = format!("INNER JOIN {} ON {}.{} = {}.{}", join_table_name, assocation_table_name, assocation_primary_key, join_table_name, stringify!(#association_foreign_key_ident));
                            query.joins(arel::serde_json::json!(full_join_string)).r#where(arel::serde_json::json!([format!("{}.{} = ?", join_table_name, stringify!(#foreign_key_ident)), attr_primary_key_json]));

                            std::result::Result::Ok(query)
                        } else {
                            std::result::Result::Err(arel::anyhow::anyhow!("Model not persisted"))
                        }
                    }
                });
                let join_association_ident = syn::Ident::new(&format!("{}_join_string", association_ident.to_string()), association_ident.span());
                final_token_stream.extend(quote::quote! {
                    pub fn #join_association_ident() -> String {
                        let assocation_table_name = #has_many_struct_ident::table_name();
                        let assocation_primary_key = #has_many_struct_ident::primary_key();

                        let join_table_name = stringify!(#join_table_ident);

                        let self_table_name = Self::table_name();
                        let self_primary_key = Self::primary_key();

                        vec![
                            format!("INNER JOIN {} ON {}.{} = {}.{}", join_table_name, assocation_table_name, assocation_primary_key, join_table_name, stringify!(#association_foreign_key_ident)),
                            format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, join_table_name, stringify!(#foreign_key_ident), self_table_name, self_primary_key),
                        ].join(" ")
                    }
                });
            }
        }
    }

    Ok(final_token_stream)
}