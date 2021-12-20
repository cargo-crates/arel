mod persisted_record_struct;

use crate::arel::generators::{fields_generator, functions_generator};
use expansion::helpers::{self, DeriveInputHelper};
use syn::{AttributeArgs, spanned::Spanned};
// use syn::{parse_quote};

pub fn generate(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    final_token_stream.extend(persisted_record_struct::generate(derive_input_helper, args));
    final_token_stream.extend(generate_struct(derive_input_helper, args));
    Ok(final_token_stream)
}

pub fn generate_struct(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let fields = derive_input_helper.get_fields()?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();

    let struct_ident = &derive_input_helper.value().ident;
    let struct_name_literal = struct_ident.to_string();
    //  User
    let arel_struct_name_literal = format!("{}", struct_name_literal);
    let arel_struct_ident = &syn::Ident::new(&arel_struct_name_literal, derive_input_helper.value().span());
    // struct UserRowRecord
    let arel_struct_row_record_name_literal = format!("{}RowRecord", struct_name_literal);
    let arel_struct_row_record_ident = &syn::Ident::new(&arel_struct_row_record_name_literal, derive_input_helper.value().span());

    let builder_fields_def = fields_generator::generate_struct_fields_define(derive_input_helper)?;
    let builder_fields_init_clauses = fields_generator::generate_struct_fields_init_clauses(derive_input_helper)?;
    let builder_functions_def_of_getters = functions_generator::accessor::generate_struct_functions_define_of_getters(derive_input_helper)?;
    let builder_functions_def_of_setters = functions_generator::accessor::generate_struct_functions_define_of_setters(derive_input_helper)?;
    let builder_functions_def_of_validates = functions_generator::validator::generate_struct_functions_define_of_validates(derive_input_helper)?;
    // let builder_functions_def = functions_generator::generate_struct_functions_define(derive_input_helper)?;
    let builder_impl_arel_functions_def = functions_generator::generate_struct_impl_arel_functions_define(derive_input_helper, args)?;

    let (impl_generics, type_generics, where_clause) = derive_input_helper.value().generics.split_for_impl();

    // primary_key_ident
    let mut primary_key_ident = syn::Ident::new("id", derive_input_helper.value().span());
    if let Some(ident) = helpers::get_macro_nested_attr_value_ident(args.iter().collect(), "primary_key", None, None)? {
        primary_key_ident = ident
    }
    // primary_attr_key_ident
    let mut primary_attr_key_ident = primary_key_ident;
    for f in fields.iter() {
        if let Some(ident) = &f.ident {
            let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
            if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), None)? {
                if rename_ident.to_string() == primary_attr_key_ident.to_string() {
                    primary_attr_key_ident = syn::Ident::new(&ident.to_string(), ident.span());
                    break;
                }
            }
        }
    }

    // model
    Ok(quote::quote! {
        // pub struct User {
        //     id: std::option::Option<i64>
        // }
        #[derive(::core::clone::Clone, ::core::fmt::Debug)]
        pub struct #arel_struct_ident #type_generics {
            persisted_row_record: std::option::Option<#arel_struct_row_record_ident #type_generics>,
            #builder_fields_def
        }
        // impl ArelAble for User {}
        #[arel::async_trait::async_trait]
        impl #impl_generics arel::ArelAble for #arel_struct_ident #type_generics #where_clause {
            type PersistedRowRecord = #arel_struct_row_record_ident #type_generics;
            #builder_impl_arel_functions_def
            // fn persisted_row_record(&self) -> std::option::Option<&Self::PersistedRowRecord>
            fn persisted_row_record(&self) -> std::option::Option<&Self::PersistedRowRecord> {
                if let std::option::Option::Some(persisted_row_record) = &self.persisted_row_record {
                    std::option::Option::Some(persisted_row_record)
                } else {
                    std::option::Option::None
                }
            }
            // fn new_from_db_row(db_row: arel::collectors::row::Row) -> arel::anyhow::Result<Self>
            // #[cfg(any(feature = "arel/sqlite", feature = "arel/mysql", feature = "arel/postgres", feature = "arel/mssql"))]
            fn new_from_db_row(db_row: arel::collectors::row::Row<Self>) -> arel::anyhow::Result<Self #type_generics> {
                let persisted_row_record = #arel_struct_row_record_ident::new_from_db_row(db_row)?;
                std::result::Result::Ok(Self {
                    #(#idents: persisted_row_record.#idents.clone(),)*
                    persisted_row_record: std::option::Option::Some(persisted_row_record),
                })
            }
            // fn assign_from_persisted_row_record(&mut self) -> arel::anyhow::Result<&mut Self>
            fn assign_from_persisted_row_record(&mut self) -> arel::anyhow::Result<&mut Self #type_generics> {
                if let std::option::Option::Some(persisted_row_record) = &self.persisted_row_record {
                    #(self.#idents = persisted_row_record.#idents.clone();)*
                }
                std::result::Result::Ok(self)
            }
            // fn assign_to_persisted_row_record(&mut self) -> arel::anyhow::Result<&mut Self>
            fn assign_to_persisted_row_record(&mut self) -> arel::anyhow::Result<&mut Self #type_generics> {
                let persisted_row_record = #arel_struct_row_record_ident #type_generics::new_from_model(self);
                self.persisted_row_record = std::option::Option::Some(persisted_row_record);
                std::result::Result::Ok(self)
            }
            // fn changed_attrs_json(&self) -> anyhow::Result<std::option::Option<arel::serde_json::Value>>
            fn changed_attrs_json(&self) -> anyhow::Result<std::option::Option<arel::serde_json::Value>> {
                let mut map = arel::serde_json::Map::new();
                let mut exists_changed = false;
                for attr in Self::attr_names().iter() {
                    if self.attr_json(attr) != self.persisted_attr_json(attr) {
                        exists_changed = true;
                        if let std::option::Option::Some(value) = self.attr_json(attr) {
                            map.insert(Self::attr_name_to_table_column_name(attr)?.to_string(), value);
                        } else {
                            map.insert(Self::attr_name_to_table_column_name(attr)?.to_string(), arel::serde_json::json!(null));
                        }
                    }
                }
                if exists_changed {
                    std::result::Result::Ok(std::option::Option::Some(arel::serde_json::Value::Object(map)))
                } else {
                    std::result::Result::Ok(std::option::Option::None)
                }
            }
            // async fn save(&mut self) -> arel::anyhow::Result<()>
            // #[cfg(any(feature = "arel/sqlite", feature = "arel/mysql", feature = "arel/postgres", feature = "arel/mssql"))]
            async fn save(&mut self) -> arel::anyhow::Result<()> {
                // validates
                self.validate()?;

                let primary_key = Self::primary_key();
                let primary_attr_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
                let primary_attr_key_value = self.persisted_attr_json(primary_attr_key);

                if let std::option::Option::Some(json) = self.changed_attrs_json()? {
                    if let Some(primary_attr_key_value) = primary_attr_key_value {
                        let mut where_clause = arel::serde_json::Map::new();
                        where_clause.insert(primary_key.to_string(), primary_attr_key_value);
                        Self::update_all(json).r#where(arel::serde_json::Value::Object(where_clause)).execute().await?;
                    } else {
                        let ret = Self::create(json).execute().await?;
                        if let std::option::Option::Some(id) = ret.last_insert_id() {
                            self.#primary_attr_key_ident = std::option::Option::Some(id.try_into()?)
                        }
                    }
                    self.assign_to_persisted_row_record()?;
                }
                std::result::Result::Ok(())
            }
            // async fn delete(&mut self) -> arel::anyhow::Result<sqlx::any::AnyQueryResult>
            // #[cfg(any(feature = "arel/sqlite", feature = "arel/mysql", feature = "arel/postgres", feature = "arel/mssql"))]
            async fn delete(&mut self) -> arel::anyhow::Result<sqlx::any::AnyQueryResult> {
                let primary_key = Self::primary_key();
                let primary_attr_key = Self::table_column_name_to_attr_name(Self::primary_key())?;
                let primary_attr_key_value = self.persisted_attr_json(primary_attr_key);

                if let std::option::Option::Some(primary_attr_key_value) = primary_attr_key_value {
                    let mut where_clause = arel::serde_json::Map::new();
                    where_clause.insert(primary_key.to_string(), primary_attr_key_value);
                    let ret = Self::delete_all(arel::serde_json::Value::Object(where_clause)).execute().await?;
                    self.persisted_row_record = std::option::Option::None;
                    std::result::Result::Ok(ret)
                } else {
                    return Err(arel::anyhow::anyhow!("Record Is Not Persisted: {:?}", self));
                }
            }
        }
        // impl User {}
        impl #impl_generics #arel_struct_ident #type_generics #where_clause {
            // pub fn new() -> Self
            pub fn new() -> Self {
                Self {
                    persisted_row_record: std::option::Option::None,
                    #builder_fields_init_clauses
                }
            }
            #builder_functions_def_of_getters
            #builder_functions_def_of_setters
            #builder_functions_def_of_validates
            // #builder_functions_def
        }
    })
}