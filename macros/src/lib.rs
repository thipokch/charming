mod deserialize_as_remote;
mod serde_auto;

extern crate proc_macro;
extern crate serde;
extern crate serde_with;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use crate::{deserialize_as_remote::*, serde_auto::*};

#[proc_macro_derive(DeserializeAsRemote)]
pub fn deserialize_as_remote(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_deserialize_as_remote(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn serde_auto(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_serde_auto(item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
