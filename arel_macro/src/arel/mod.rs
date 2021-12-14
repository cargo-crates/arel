pub mod generators;
mod model_struct;

use expansion::helpers::{DeriveInputHelper};

use proc_macro::TokenStream;
use syn::{AttributeArgs};
// use syn::{parse_quote};

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
    model_struct::generate(derive_input_helper, args)
}