use darling::{FromDeriveInput, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;

pub fn expand_deserialize_as_remote(input: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let container_name = &input.ident;

    #[derive(Debug, FromDeriveInput)]
    #[darling(attributes(serde))]
    struct SerdeRemote {
        remote: String,
    }

    let remote_name = match SerdeRemote::from_derive_input(&input) {
        Ok(serde) => syn::Path::from_string(&serde.remote)?,
        Err(err) => return Err(err.into()),
    };

    let gen = quote! {
        impl<'de> serde_with::DeserializeAs<'de, #remote_name> for #container_name {
            fn deserialize_as<D>(deserializer: D) -> Result<#remote_name, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #container_name::deserialize(deserializer)
            }
        }
    };

    Ok(gen.into())
}
