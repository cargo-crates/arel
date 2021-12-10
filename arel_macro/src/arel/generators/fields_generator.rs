use expansion::helpers::{self, DeriveInputHelper};
pub fn generate_struct_fields_define(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fileds = derive_input_helper.get_fields()?;

    let idents: Vec<_> = fileds.iter().map(|f| &f.ident).collect();
    // let types: Vec<_> = fileds.iter().map(|f| &f.ty).collect();

    let types: Vec<_> = fileds.iter().map(|f| {
        // &f.ty
        if let Some(ty_inner_type) = helpers::get_type_inner_type_ident(&f.ty, "Option") {
            quote::quote! {
                std::option::Option<#ty_inner_type>
            }
        } else if let Some(_) = helpers::get_type_inner_type_ident(&f.ty, "Vec") {
            let origin_type = &f.ty;
            quote::quote! {
                #origin_type
            }
        } else {
            let origin_type = &f.ty;
            quote::quote! {
                std::option::Option<#origin_type>
            }
        }
    }).collect();
    Ok(quote::quote! {
        #(#idents: #types),*
    })
}

pub fn generate_struct_fields_init_clauses(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fileds = derive_input_helper.get_fields()?;

    let idents: Vec<_> = fileds.iter().map(|f| &f.ident).collect();

    // Ok(quote::quote! {
    //     #(#idents: std::option::Option::None),*
    // })

    let types: Vec<_> = fileds.iter().map(|f| &f.ty).collect();

    let mut final_token_stream = proc_macro2::TokenStream::new();
    for (ident, r#type) in idents.iter().zip(types.iter()) {
        let token_stream_piece;
        if let Some(_) = helpers::get_type_inner_type_ident(r#type, "Vec") {
            token_stream_piece = quote::quote! {
                            #ident: std::vec::Vec::new(),
                        };
        } else {
            token_stream_piece = quote::quote! {
                            #ident: std::option::Option::None,
                        };
        }
        final_token_stream.extend(token_stream_piece);
    }
    Ok(final_token_stream)
}