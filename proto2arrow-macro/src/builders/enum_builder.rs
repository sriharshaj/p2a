use heck::ToSnakeCase;
use quote::quote;
use syn::spanned::Spanned;

use crate::generators;
use crate::type_utils as common;

fn generate_builder_fields<'a>(
    vis: &syn::Visibility,
    variant_ident: &'a syn::Ident,
    variant_fields: &syn::Fields,
) -> Result<common::EnumVariant<'a>, syn::Error> {
    if variant_fields.len() != 1 {
        return Err(syn::Error::new(
            variant_ident.span(),
            "Enum variants must have exactly one field. This macro is designed for prost-generated enums from protobuf oneof fields",
        ));
    }

    let mut field = variant_fields.iter().next().unwrap().clone();

    field.attrs = Vec::new();
    field.vis = vis.clone();
    field.colon_token = Some(syn::token::Colon::default());
    field.ident = Some(syn::Ident::new(
        &variant_ident.to_string().to_snake_case(),
        variant_ident.span(),
    ));

    let (ty, cardinality) = match field.ty {
        syn::Type::Path(type_path) => common::parse_type(type_path)?,
        _ => {
            return Err(syn::Error::new(
                field.ty.span(),
                "Expected protobuf compatible type",
            ));
        }
    };
    field.ty = ty;

    Ok(common::EnumVariant {
        ident: variant_ident,
        fc: common::FieldWithCardinality { field, cardinality },
    })
}

pub fn enum_arrow_builder(message: syn::ItemEnum) -> Result<proc_macro::TokenStream, syn::Error> {
    let message_instance_ident: syn::Ident = syn::parse_quote!(record);
    let builder_ident = syn::Ident::new(&format!("{}Builder", message.ident), message.ident.span());
    let builder_vis = &message.vis;

    let mut variants: Vec<common::EnumVariant> = Vec::with_capacity(message.variants.len());
    for v in &message.variants {
        //TODO: change later to proper code
        if v.fields.is_empty() {
            return Ok((quote! {
                #message
            })
            .into());
        }
        variants.push(generate_builder_fields(builder_vis, &v.ident, &v.fields)?);
    }

    let append_value_fn = generators::generate_enum_append_value_fn(
        builder_vis,
        &message_instance_ident,
        &message.ident,
        &variants,
    );

    let fc_list: Vec<common::FieldWithCardinality> = variants.into_iter().map(|v| v.fc).collect();

    let append_null_fn = generators::generate_append_null_fn(builder_vis, &fc_list);
    let append_option_fn =
        generators::generate_append_option_fn(builder_vis, &message_instance_ident, &message.ident);

    let finish_fn =
        generators::generate_finish_fn(builder_vis, &fc_list, common::MessageDataType::Enum);
    let finish_cloned_fn =
        generators::generate_finish_cloned_fn(builder_vis, &fc_list, common::MessageDataType::Enum);

    let builder_impl: syn::ItemImpl = syn::parse_quote! {
        impl #builder_ident{
            #append_value_fn
            #append_null_fn
            #append_option_fn
            #finish_fn
            #finish_cloned_fn
        }
    };
    let default_trait_impl = generators::generate_default_trait_impl(&builder_ident, &fc_list);
    let extend_trait_impl = generators::generate_extend_trait_impl(&message.ident, &builder_ident);
    let array_builder_trait_impl = generators::generate_array_builder_trait_impl(&builder_ident);
    let builder_fields = fc_list
        .into_iter()
        .map(|f| f.field)
        .collect::<Vec<syn::Field>>();

    Ok((quote! {
    #message

    #[derive(Debug)]
    #builder_vis struct #builder_ident {
        #(#builder_fields,)*

        _nulls: ::arrow::array::NullBufferBuilder,
    }

    #default_trait_impl
    #extend_trait_impl

    #builder_impl
    #array_builder_trait_impl
    })
    .into())
}
