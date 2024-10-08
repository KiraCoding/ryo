use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Error};

pub(crate) fn derive_union_input(input: DeriveInput) -> TokenStream {
    let _data_union = match &input.data {
        Data::Enum(data_union) => data_union,
        _ => {
            return Error::new(input.span(), "Not a union")
                .into_compile_error()
                .into()
        }
    };

    todo!()
}
