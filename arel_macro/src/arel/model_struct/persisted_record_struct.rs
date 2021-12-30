use crate::arel::generators::{fields_generator, functions_generator};
use expansion::helpers::{self, DeriveInputHelper};

use syn::{AttributeArgs, spanned::Spanned};
// use syn::{parse_quote};

pub fn generate(derive_input_helper: &DeriveInputHelper, _args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
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
    let builder_functions_def_of_getters = functions_generator::accessor::generate_struct_functions_define_of_getters(derive_input_helper)?;

    let (impl_generics, type_generics, where_clause) = derive_input_helper.value().generics.split_for_impl();

    let mut init_from_db_row_init_token_streams: Vec<_> = vec![];
    for f in fields.iter() {
        if let Some(ident) = &f.ident {
            let table_rename_ident;
            let mut table_column_ident = ident;
            let metas = helpers::parse_attrs_to_metas(&f.attrs)?;
            if let Some(rename_ident) = helpers::get_macro_attr_value_ident(metas.iter().collect(), "table_column_name", Some(vec!["arel"]), None)? {
                table_rename_ident = rename_ident;
                table_column_ident = &table_rename_ident;
            }
            let mut r#type = &f.ty;
            if let Some(inner_type) = helpers::get_type_inner_type_ident(r#type, "Option") {
                r#type = inner_type;
            }
            init_from_db_row_init_token_streams.push(quote::quote! {
                #ident: if let std::result::Result::Ok(value) = db_row.sqlx_row.try_get::<#r#type, _>(stringify!(#table_column_ident)) { std::option::Option::Some(value) } else { std::option::Option::None },
            })
        }
    }
    Ok(quote::quote! {
        // pub struct UserRowRecord{}
        #[derive(std::clone::Clone, std::fmt::Debug)]
        pub struct #arel_struct_row_record_ident #type_generics {
            #builder_fields_def
        }
        // impl UserRowRecord
        impl #impl_generics #arel_struct_row_record_ident #type_generics #where_clause {
            // fn new_from_db_row(db_row: sqlx::any::AnyRow) -> arel::anyhow::Result<Self>
            // #[cfg(any(feature = "arel/sqlite", feature = "arel/mysql", feature = "arel/postgres", feature = "arel/mssql"))]
            fn new_from_db_row(db_row: arel::collectors::row::Row<#struct_ident>) -> arel::anyhow::Result<Self #type_generics> {
                std::result::Result::Ok(Self {
                    #(#init_from_db_row_init_token_streams)*
                })
            }
            // fn new_from_model(model: &User) -> Self
            fn new_from_model(model: &#arel_struct_ident #type_generics) -> Self #type_generics {
                Self {
                    #(#idents: model.#idents.clone(),)*
                }
            }
            #builder_functions_def_of_getters
        }
    })
}