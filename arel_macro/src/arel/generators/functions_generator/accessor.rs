use expansion::helpers::{self, DeriveInputHelper};

pub fn generate_struct_functions_define_of_getters(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fields = derive_input_helper.get_fields()?;

    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_token_stream = proc_macro2::TokenStream::new();

    // getters
    for (ident, r#type) in idents.iter().zip(types.iter()) {
        if let Some(_) = helpers::get_type_inner_type_ident(r#type, "Vec") {
            final_token_stream.extend(quote::quote! {
                    fn #ident(&self) -> &#r#type {
                        &self.#ident
                    }
                });
        } else if let Some(inner_type) = helpers::get_type_inner_type_ident(r#type, "Option") {
            final_token_stream.extend(quote::quote! {
                    fn #ident(&self) -> std::option::Option<&#inner_type> {
                        if let Some(value) = &self.#ident {
                            std::option::Option::Some(value)
                        } else {
                            std::option::Option::None
                        }
                    }
                });
        } else {
            final_token_stream.extend(quote::quote! {
                    fn #ident(&self) -> std::option::Option<&#r#type> {
                        if let Some(value) = &self.#ident {
                            std::option::Option::Some(value)
                        } else {
                            std::option::Option::None
                        }
                    }
                });
        }
    }
    Ok(final_token_stream)
}

pub fn generate_struct_functions_define_of_setters(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fields = derive_input_helper.get_fields()?;

    // let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    // let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let mut final_token_stream = proc_macro2::TokenStream::new();

    let setter_token_streams: Vec<_> = fields.iter().filter_map(|f| {
        let r#type = &f.ty;
        if let Some(ident) = &f.ident {
            let set_name_ident_name = format!("set_{}", ident.to_string());
            let set_name_ident = &syn::Ident::new(&set_name_ident_name, ident.span());
            if helpers::get_type_inner_type_ident(r#type, "Vec").is_some() || helpers::get_type_inner_type_ident(r#type, "Option").is_some() {
                Some(quote::quote! {
                        fn #set_name_ident(&mut self, #ident: #r#type) -> &mut Self {
                            self.#ident = #ident;
                            self
                        }
                    })
            }  else {
                // T -> T, Option<T> -> T
                // let r#type = if let Some(inner_type) = helpers::get_type_inner_type_ident(r#type, "Option") { inner_type } else { r#type };
                Some(quote::quote! {
                        fn #set_name_ident(&mut self, #ident: #r#type) -> &mut Self {
                            self.#ident = std::option::Option::Some(#ident);
                            self
                        }
                    })
            }
        } else {
            None
        }
    }).collect();
    // setters
    for piece_token_stream in setter_token_streams {
        final_token_stream.extend(piece_token_stream);
    }
    Ok(final_token_stream)
}