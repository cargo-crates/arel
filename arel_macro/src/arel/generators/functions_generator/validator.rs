use expansion::helpers::{self, DeriveInputHelper};

pub fn generate_struct_functions_define_of_validates(derive_input_helper: &DeriveInputHelper) -> syn::Result<proc_macro2::TokenStream> {
    let fields = derive_input_helper.get_fields()?;

    let mut final_token_stream = proc_macro2::TokenStream::new();
    // pub fn validate(&self) -> arel::anyhow::Result<()> {}
    {
        let segments: Vec<_> = fields.iter().filter(|f| {
            helpers::get_type_inner_type_ident(&f.ty, "Option").is_none() && helpers::get_type_inner_type_ident(&f.ty, "Vec").is_none()
        }).map(|f| {
            let ident = &f.ident;
            quote::quote! {
                    if self.#ident.is_none() {
                        return Err(arel::anyhow::anyhow!("{} Not Allow None", stringify!(#ident)));
                    }
                }
        }).collect();
        let validations_token_stream = quote::quote! {
            pub fn validate(&self) -> arel::anyhow::Result<()> {
                #(#segments)*
                Ok(())
            }
        };
        final_token_stream.extend(validations_token_stream);
    }
    Ok(final_token_stream)
}