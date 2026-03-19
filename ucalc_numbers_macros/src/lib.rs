use proc_macro::TokenStream;
use quote::quote;
#[proc_macro]
pub fn generate_types(_: TokenStream) -> TokenStream {
    quote! {}.into()
}
