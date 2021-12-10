mod generators;

use expansion::helpers::{DeriveInputHelper};

use proc_macro::TokenStream;
use syn::{AttributeArgs, spanned::Spanned};
// use syn::{parse_quote};
use quote;

pub fn create_arel(args: TokenStream, input: TokenStream) -> TokenStream {
    // AttributeArgs 及为 Vec<NestedMeta>类型的语法树节点
    let args = syn::parse_macro_input!(args as AttributeArgs);
    let derive_input = syn::parse_macro_input!(input as syn::DeriveInput);

    let derive_input_helper = DeriveInputHelper::new(derive_input);

    match do_expand(&derive_input_helper, &args) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &derive_input_helper.value().ident;
    let struct_name_literal = struct_ident.to_string();
    let arel_struct_name_literal = format!("{}", struct_name_literal);
    let arel_struct_ident = &syn::Ident::new(&arel_struct_name_literal, derive_input_helper.value().span());

    let builder_fields_def = generators::fields_generator::generate_struct_fields_define(derive_input_helper)?;
    let builder_fields_init_clauses = generators::fields_generator::generate_struct_fields_init_clauses(derive_input_helper)?;
    let builder_functions_def = generators::functions_generator::generate_struct_functions_define(derive_input_helper)?;
    let builder_impl_arel_functions_def = generators::functions_generator::generate_struct_impl_arel_functions_define(derive_input_helper, args)?;

    let (impl_generics, type_generics, where_clause) = derive_input_helper.value().generics.split_for_impl();

    let ret = quote::quote! {
        // UserArel
        #[derive(::core::clone::Clone, ::core::fmt::Debug)]
        pub struct #arel_struct_ident #type_generics {
            #builder_fields_def
        }

        impl #impl_generics arel::ArelAble for #arel_struct_ident #type_generics #where_clause {
            #builder_impl_arel_functions_def
        }

        impl #impl_generics #arel_struct_ident #type_generics #where_clause {
            pub fn new() -> Self {
                Self {
                    #builder_fields_init_clauses
                }
            }
            #builder_functions_def
        }
    };
    Ok(ret)
}