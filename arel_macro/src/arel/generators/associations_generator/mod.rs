pub mod has_many_generator;
pub mod has_one_generator;
pub mod belongs_to_generator;
pub mod has_and_belongs_to_many_generator;

#[allow(unused_imports)]
use expansion::helpers::{self, DeriveInputHelper};
#[allow(unused_imports)]
use syn::{AttributeArgs, spanned::Spanned};

pub fn generate_associations(derive_input_helper: &DeriveInputHelper, args: &AttributeArgs) -> syn::Result<proc_macro2::TokenStream> {
    let mut final_token_stream = proc_macro2::TokenStream::new();
    final_token_stream.extend(has_many_generator::generate_has_many_associations(derive_input_helper, args)?);
    final_token_stream.extend(has_one_generator::generate_has_one_associations(derive_input_helper, args)?);
    final_token_stream.extend(belongs_to_generator::generate_belongs_to_associations(derive_input_helper, args)?);
    final_token_stream.extend(has_and_belongs_to_many_generator::generate_has_and_belongs_to_many_associations(derive_input_helper, args)?);
    Ok(final_token_stream)
}



