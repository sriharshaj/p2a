use crate::type_utils as common;

pub fn generate_default_trait_impl(
    builder_ident: &syn::Ident,
    fc_list: &[common::FieldWithCardinality],
) -> syn::ItemImpl {
    let default_exprs: Vec<syn::FieldValue> = fc_list
        .iter()
        .map(|fg| match fg.cardinality {
            common::FieldCardinality::Map => {
                let ident = fg.field.ident.clone().unwrap();
                syn::FieldValue {
                    attrs: Vec::new(),
                    member: syn::Member::Named(ident),
                    colon_token: Some(syn::token::Colon::default()),
                    expr: syn::parse_quote!(::arrow::array::MapBuilder::new(
                        None,
                        Default::default(),
                        Default::default()
                    )),
                }
            }
            _ => {
                let ident = fg.field.ident.clone().unwrap();
                syn::FieldValue {
                    attrs: Vec::new(),
                    member: syn::Member::Named(ident),
                    colon_token: Some(syn::token::Colon::default()),
                    expr: syn::parse_quote!(Default::default()),
                }
            }
        })
        .collect();
    syn::parse_quote! {
        impl Default for #builder_ident {
            fn default() -> Self {
                #builder_ident {
                    #(#default_exprs,)*

                    _nulls: ::arrow::array::NullBufferBuilder::new(0),
                }
            }
        }
    }
}

pub fn generate_extend_trait_impl(
    message_ident: &syn::Ident,
    builder_ident: &syn::Ident,
) -> syn::ItemImpl {
    syn::parse_quote! {
        impl Extend<Option<#message_ident>> for #builder_ident {
            fn extend<T: IntoIterator<Item = Option<#message_ident>>>(&mut self, iter: T) {
                iter.into_iter().for_each(|r| self.append_option(r));
            }
        }
    }
}

pub fn generate_array_builder_trait_impl(builder_ident: &syn::Ident) -> syn::ItemImpl {
    syn::parse_quote! {
        impl ::arrow::array::ArrayBuilder for #builder_ident {
            fn len(&self) -> usize {
                self._nulls.len()
            }

            fn finish(&mut self) -> ::arrow::array::ArrayRef {
                ::std::sync::Arc::new(self.finish())
            }

            fn finish_cloned(&self) -> ::arrow::array::ArrayRef {
                ::std::sync::Arc::new(self.finish_cloned())
            }

            fn as_any(&self) -> &dyn ::std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn ::std::any::Any {
                self
            }

            fn into_box_any(self: Box<Self>) -> Box<dyn ::std::any::Any> {
                self
            }
        }
    }
}
