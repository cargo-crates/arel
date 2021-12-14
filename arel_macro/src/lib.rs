pub(crate) mod arel;

use proc_macro::TokenStream;

/// #[derive(arel::Arel)]
/// #[arel(table_name="users")]
/// struct User {
///     id: usize,
///    desc: String,
///     published: Option<bool>,
///     // ids: Vec<T>,
/// }
// #[proc_macro_derive(Arel, attributes(arel))]
// pub fn derive_arel(input: TokenStream) -> TokenStream {
//     arel::derive::derive(input)
// }


/// #[arel::arel(table_name="users")]
/// struct User {
///     id: usize,
///     desc: String,
///     published: Option<bool>,
///     // ids: Vec<T>,
/// }
#[proc_macro_attribute]
pub fn arel(args: TokenStream, input: TokenStream) -> TokenStream {
    arel::create_arel(args, input)
}