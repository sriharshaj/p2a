use proc_macro::TokenStream;
use syn::parse_macro_input;

mod builders;
mod generators;
mod type_utils;

#[proc_macro_attribute]
pub fn derive_builder(_metadata: TokenStream, item: TokenStream) -> TokenStream {
    let derived = parse_macro_input!(item as syn::DeriveInput);
    match derived.data {
        syn::Data::Enum(data) => builders::enum_arrow_builder(syn::ItemEnum {
            attrs: derived.attrs,
            vis: derived.vis,
            enum_token: data.enum_token,
            ident: derived.ident,
            generics: derived.generics,
            brace_token: data.brace_token,
            variants: data.variants,
        })
        .unwrap_or_else(|e| e.to_compile_error().into()),
        syn::Data::Struct(data) => builders::struct_arrow_builder(syn::ItemStruct {
            attrs: derived.attrs,
            vis: derived.vis,
            struct_token: data.struct_token,
            ident: derived.ident,
            generics: derived.generics,
            fields: data.fields,
            semi_token: data.semi_token,
        })
        .unwrap_or_else(|e| e.to_compile_error().into()),
        syn::Data::Union(_) => syn::Error::new(
            derived.ident.span(),
            "Union types are not supported by proto2arrow. Only structs and enums are supported.",
        )
        .to_compile_error()
        .into(),
    }
}
