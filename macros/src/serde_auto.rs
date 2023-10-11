use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_serde_auto(item: TokenStream) -> syn::Result<TokenStream> {
    let gen = quote! {
        #[serde_with::serde_as]
        #[serde_with::apply(
            Option => #[serde(default, skip_serializing_if = "Option::is_none")],
            Vec => #[serde(default, skip_serializing_if = "Vec::is_empty")],
            DataFrame => #[serde(default, skip_serializing_if = "Vec::is_empty")],
        )]
        #item
    };

    Ok(gen.into())
}
