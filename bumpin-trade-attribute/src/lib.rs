// src/lib.rs
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Make #[event] compatible with IDL Build
#[proc_macro_attribute]
pub fn bumpin_zero_copy_unsafe(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let struct_name = &input.ident;
    let struct_attrs = &input.attrs;
    let struct_vis = &input.vis;
    let struct_fields = &input.fields;

    #[cfg(not(feature = "idl-build"))]
    let expanded = quote! {
        #[zero_copy(unsafe)]
        #[repr(C)]
        #[derive(AnchorSerialize, AnchorDeserialize, PartialEq, Default, Debug, Eq)]
        #(#struct_attrs)*
        #struct_vis struct #struct_name #struct_fields
    };

    #[cfg(feature = "idl-build")]
    let expanded = quote! {
        #[repr(C)]
        #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Default, Debug, Eq)]
        #(#struct_attrs)*
        #struct_vis struct #struct_name #struct_fields
    };

    TokenStream::from(expanded)
}
