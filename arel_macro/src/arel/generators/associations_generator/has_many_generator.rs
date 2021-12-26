use expansion::helpers::{self, DeriveInputHelper};
#[allow(unused_imports)]
use syn::{AttributeArgs, spanned::Spanned};

use super::{has_and_belongs_to_many_generator, has_one_generator};


pub fn get_has_many_args_vec(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<Option<Vec<Vec<syn::NestedMeta>>>> {
    let mut total_vec = vec![];
    let metas = helpers::parse_attrs_to_metas(&derive_input_helper.value().attrs)?;
    if let Some(mut vec) = helpers::get_namespace_nested_metas_vec_from_metas(metas.iter().collect::<Vec<_>>(), vec!["has_many"])? {
        total_vec.append(&mut vec)
    }
    if let Some(mut vec) = helpers::get_namespace_nested_metas_vec_from_nested_metas(args.iter().collect(), vec!["has_many"])? {
        total_vec.append(&mut vec)
    }
    if total_vec.len() > 0 {
        Ok(Some(total_vec.into_iter().map(|i| i.into_iter().map(|i| i.clone()).collect::<Vec<_>>()).collect::<Vec<_>>()))
    } else {
        Ok(None)
    }
}

pub fn handle_association_attributes(has_many_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident)>> {
    let self_struct_ident = &derive_input_helper.value().ident;
    let self_struct_name = format!("{}", self_struct_ident.to_string());
    let self_struct_ident = &syn::Ident::new(&self_struct_name, derive_input_helper.value().span());

    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_many_args.get(0).unwrap() {
        let association_name = name.value();
        let association_ident = syn::Ident::new(&association_name, name.span());
        if let Some(has_many_struct_ident) = helpers::get_macro_nested_attr_value_ident(has_many_args.clone(), "struct", None, None)? {
            let foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&self_struct_ident.to_string()));
            let mut foreign_key_ident = syn::Ident::new(&foreign_key, has_many_struct_ident.span());
            if let Some(custom_foreign_key_ident) = helpers::get_macro_nested_attr_value_ident(has_many_args.clone(), "foreign_key", None, None)? {
                // foreign_key = custom_foreign_key_ident.to_string();
                foreign_key_ident = custom_foreign_key_ident;
            }
            return Ok(Some((association_ident, has_many_struct_ident, foreign_key_ident)))
        } else {
            return Err(syn::Error::new_spanned(name, "loss attr: struct".to_string()))
        }
    }
    Ok(None)
}

pub fn generate_has_many_associations(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    if let Some(has_many_args_vec) = get_has_many_args_vec(derive_input_helper, args)? {
        for has_many_args in has_many_args_vec.into_iter() {
            if let Some(has_many_through_token_stream) = generate_has_many_through_associations(has_many_args.iter().collect(), derive_input_helper, args)? {
                final_token_stream.extend(has_many_through_token_stream);
            } else if let Some((association_ident, has_many_struct_ident, foreign_key_ident)) = handle_association_attributes(has_many_args.iter().collect(), derive_input_helper, args)? {
                final_token_stream.extend(quote::quote! {
                    pub fn #association_ident(&self) -> arel::anyhow::Result<arel::table::Table<#has_many_struct_ident>> {
                        let assocation_table_name = #has_many_struct_ident::table_name();
                        let assocation_foregin_key = stringify!(#foreign_key_ident);
                        // check foreign_key exists
                        let association_table_columns = #has_many_struct_ident::table_column_names();
                        if !association_table_columns.contains(&assocation_foregin_key) {
                            panic!("has_many foreign_key({}) Not In Table {} Columns: {:?}", assocation_foregin_key, assocation_table_name, association_table_columns);
                        }

                        let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
                        if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
                            let mut map = arel::serde_json::Map::new();
                            map.insert(assocation_foregin_key.to_string(), attr_primary_key_json);
                            let mut query = #has_many_struct_ident::query();
                            query.r#where(arel::serde_json::Value::Object(map));
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
                        let assocation_foregin_key = stringify!(#foreign_key_ident);
                        // check foreign_key exists
                        let association_table_columns = #has_many_struct_ident::table_column_names();
                        if !association_table_columns.contains(&assocation_foregin_key) {
                            panic!("has_many foreign_key({}) Not In Table {} Columns: {:?}", assocation_foregin_key, assocation_table_name, association_table_columns);
                        }

                        let self_table_name = Self::table_name();
                        let self_primary_key = Self::primary_key();
                        format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, assocation_table_name, assocation_foregin_key, self_table_name, self_primary_key)
                    }
                });

            }
        }
    }
    Ok(final_token_stream)
}

//  a through b, b through c => [b_args, c_args]
fn get_through_nested_metas_recursion(through_name: String, derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<Vec<(String, Vec<syn::NestedMeta>)>> {
    let mut through_recursion = vec![];
    let mut find_item_args = None;
    let mut association_type = "".to_string();
    if let Some(has_and_belongs_to_args_vec) = has_and_belongs_to_many_generator::get_has_and_belongs_to_many_args_vec(derive_input_helper, args)? {
        for has_and_belongs_to_args in has_and_belongs_to_args_vec.into_iter() {
            if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_and_belongs_to_args.get(0).unwrap() {
                if name.value() == through_name {
                    find_item_args = Some(has_and_belongs_to_args);
                    association_type = "has_and_belongs_to_many".to_string();
                    break;
                }
            }
        }
    }
    if let Some(has_many_args_vec) = get_has_many_args_vec(derive_input_helper, args)? {
        for has_many_args in has_many_args_vec.into_iter() {
            if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_many_args.get(0).unwrap() {
                if name.value() == through_name {
                    find_item_args = Some(has_many_args);
                    association_type = "has_many".to_string();
                    break;
                }
            }
        }
    }
    if let Some(has_one_args_vec) = has_one_generator::get_has_one_args_vec(derive_input_helper, args)? {
        for has_one_args in has_one_args_vec.into_iter() {
            if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_one_args.get(0).unwrap() {
                if name.value() == through_name {
                    find_item_args = Some(has_one_args);
                    association_type = "has_one".to_string();
                    break;
                }
            }
        }
    }

    if let Some(find_item_args) = find_item_args {
        through_recursion.push((association_type, find_item_args.clone()));
        if let Some(inner_through_ident) = helpers::get_macro_nested_attr_value_ident(find_item_args.iter().collect(), "through", None, None)? {
            through_recursion.append(&mut get_through_nested_metas_recursion(inner_through_ident.to_string(), derive_input_helper, args)?);
        }
    }

    Ok(through_recursion)
}

pub fn handle_association_through_attributes(through_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs) -> syn::Result<Option<(syn::Ident, syn::Ident, syn::Ident)>> {
    let self_struct_ident = &derive_input_helper.value().ident;
    let self_struct_name = format!("{}", self_struct_ident.to_string());
    let self_struct_ident = &syn::Ident::new(&self_struct_name, derive_input_helper.value().span());

    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = through_args.get(0).unwrap() {
        let through_association_name = name.value();
        if let Some(through_struct_ident) = helpers::get_macro_nested_attr_value_ident(through_args.clone(), "struct", None, None)? {
            // let through_struct_name = through_struct_ident.to_string();
            let through_foreign_key = format!("{}_id", inflector::cases::snakecase::to_snake_case(&self_struct_ident.to_string()));
            let mut through_foreign_key_ident = syn::Ident::new(&through_foreign_key, through_struct_ident.span());
            if let Some(custom_foreign_key_ident) = helpers::get_macro_nested_attr_value_ident(through_args.clone(), "foreign_key", None, None)? {
                // foreign_key = custom_foreign_key_ident.to_string();
                through_foreign_key_ident = custom_foreign_key_ident;
            }
            let mut through_source_ident = syn::Ident::new(&through_association_name, name.span());
            if let Some(source_ident) = helpers::get_macro_nested_attr_value_ident(through_args.clone(), "source", None, None)? {
                through_source_ident = source_ident
            }
            return Ok(Some((through_struct_ident, through_foreign_key_ident, through_source_ident)));
        } else {
            return Err(syn::Error::new_spanned(name, "loss attr: struct".to_string()))
        }
    }
    Ok(None)
}

pub fn generate_has_many_through_associations(has_many_args: Vec<&syn::NestedMeta>, derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<Option<proc_macro2::TokenStream>> {
    if let syn::NestedMeta::Lit(syn::Lit::Str(name)) = has_many_args.get(0).unwrap() {
        let association_name = name.value();
        let association_ident = syn::Ident::new(&association_name, name.span());
        if let Some(has_many_struct_ident) = helpers::get_macro_nested_attr_value_ident(has_many_args.clone(), "struct", None, None)? {
            if let Some(through_ident) = helpers::get_macro_nested_attr_value_ident(has_many_args.clone(), "through", None, None)? {
                let through_args_vec = get_through_nested_metas_recursion(through_ident.to_string(), derive_input_helper, args)?;
                let mut through_recursion_join_strings_token_stream = vec![];

                let mut last_through_struct_ident = None;
                let mut last_through_foreign_key_ident = None;
                let mut last_association_through_args_data: Option<(String, Vec<syn::NestedMeta>)> = None;
                // source字段指定内部的关联名称，默认直接使用自己的关联名称作为内部的关联名称
                let mut in_through_association_ident = association_ident.clone();
                if let Some(source_ident) = helpers::get_macro_nested_attr_value_ident(has_many_args.clone(), "source", None, None)? {
                    in_through_association_ident = source_ident
                }

                // let through_args_vec_length = through_args_vec.len();
                for (association_type, through_args) in through_args_vec.into_iter() {
                    if let Some((through_struct_ident, through_foreign_key_ident, through_source_ident)) = handle_association_through_attributes(through_args.iter().collect(), derive_input_helper, args)? {
                        last_through_struct_ident = Some(through_struct_ident.clone());
                        last_through_foreign_key_ident = Some(through_foreign_key_ident.clone());
                        last_association_through_args_data = Some((association_type.clone(), through_args.clone()));

                        let through_join_string_ident = syn::Ident::new(&format!("{}_join_string", in_through_association_ident.to_string()), in_through_association_ident.span());
                        through_recursion_join_strings_token_stream.push(quote::quote! {
                            #through_struct_ident::#through_join_string_ident(),
                        });

                        in_through_association_ident = through_source_ident;
                    }
                }
                if let (Some(last_through_struct_ident), Some(last_through_foreign_key_ident)) = (last_through_struct_ident, last_through_foreign_key_ident) {
                    let mut final_token_stream = proc_macro2::TokenStream::new();
                    if let Some((association_type, through_args)) = last_association_through_args_data {
                        if association_type == "has_and_belongs_to_many" {
                            if let Some((_, _, foreign_key_ident, association_foreign_key_ident, join_table_ident)) = has_and_belongs_to_many_generator::handle_association_attributes(through_args.iter().collect(), derive_input_helper, args)? {
                                // println!("{},{}, {}, {}, {}", association_ident.to_string(), has_many_struct_ident.to_string(), foreign_key_ident.to_string(), association_foreign_key_ident.to_string(), join_table_ident.to_string());
                                final_token_stream.extend(quote::quote! {
                                    pub fn #association_ident(&self) -> arel::anyhow::Result<arel::table::Table<#has_many_struct_ident>> {
                                        let last_struct_table_name = #last_through_struct_ident::table_name();
                                        let last_struct_primary_key = #last_through_struct_ident::primary_key();

                                        let mut join_strings = vec![#(#through_recursion_join_strings_token_stream)*];
                                        join_strings.push(format!("INNER JOIN {} ON {}.{} = {}.{}", stringify!(#join_table_ident), last_struct_table_name, last_struct_primary_key, stringify!(#join_table_ident), stringify!(#association_foreign_key_ident)));
                                        let full_join_string = join_strings.join(" ");

                                        let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
                                        if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
                                            let mut query = #has_many_struct_ident::query();
                                            query.joins(arel::serde_json::json!(full_join_string)).r#where(arel::serde_json::json!([format!("{}.{} = ?", stringify!(#join_table_ident), stringify!(#foreign_key_ident).to_string()), attr_primary_key_json]));
                                            std::result::Result::Ok(query)
                                        } else {
                                            std::result::Result::Err(arel::anyhow::anyhow!("Model not persisted"))
                                        }
                                    }
                                });
                                let join_association_ident = syn::Ident::new(&format!("{}_join_string", association_ident.to_string()), association_ident.span());
                                final_token_stream.extend(quote::quote! {
                                    pub fn #join_association_ident() -> String {
                                        let last_struct_table_name = #last_through_struct_ident::table_name();
                                        let last_struct_primary_key = #last_through_struct_ident::primary_key();

                                        let mut join_strings = vec![#(#through_recursion_join_strings_token_stream)*];
                                        join_strings.push(format!("INNER JOIN {} ON {}.{} = {}.{}", stringify!(#join_table_ident), last_struct_table_name, last_struct_primary_key, stringify!(#join_table_ident), stringify!(#association_foreign_key_ident)));

                                        let self_table_name = Self::table_name();
                                        let self_primary_key = Self::primary_key();

                                        join_strings.push(format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, stringify!(#join_table_ident), stringify!(#foreign_key_ident), self_table_name, self_primary_key));
                                        join_strings.join(" ")
                                    }
                                });
                            }
                        } else {
                            final_token_stream.extend(quote::quote! {
                                pub fn #association_ident(&self) -> arel::anyhow::Result<arel::table::Table<#has_many_struct_ident>> {
                                    let join_strings = vec![#(#through_recursion_join_strings_token_stream)*];
                                    let full_join_string = join_strings.join(" ");

                                    let attr_primary_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
                                    if let Some(attr_primary_key_json) = self.persisted_attr_json(attr_primary_key) {
                                        let mut query = #has_many_struct_ident::query();
                                        let last_struct_table_name = #last_through_struct_ident::table_name();
                                        query.joins(arel::serde_json::json!(full_join_string)).r#where(arel::serde_json::json!([format!("{}.{} = ?", last_struct_table_name, stringify!(#last_through_foreign_key_ident).to_string()), attr_primary_key_json]));
                                        std::result::Result::Ok(query)
                                    } else {
                                        std::result::Result::Err(arel::anyhow::anyhow!("Model not persisted"))
                                    }
                                }
                            });
                            let join_association_ident = syn::Ident::new(&format!("{}_join_string", association_ident.to_string()), association_ident.span());
                            final_token_stream.extend(quote::quote! {
                                pub fn #join_association_ident() -> String {
                                    let mut join_strings = vec![#(#through_recursion_join_strings_token_stream)*];

                                    let assocation_table_name = #last_through_struct_ident::table_name();
                                    let assocation_foregin_key = stringify!(#last_through_foreign_key_ident);
                                    let self_table_name = Self::table_name();
                                    let self_primary_key = Self::primary_key();

                                    join_strings.push(format!("INNER JOIN {} ON {}.{} = {}.{}", self_table_name, assocation_table_name, assocation_foregin_key, self_table_name, self_primary_key));
                                    join_strings.join(" ")
                                }
                            });
                        }
                        return Ok(Some(final_token_stream))
                    }
                }
            }
        }
    }
    Ok(None)
}