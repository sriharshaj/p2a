use crate::generators;
use crate::type_utils as common;
use syn::spanned::Spanned;

fn generate_builder_fields(
    item_struct: &syn::ItemStruct,
) -> Result<Vec<common::FieldWithCardinality>, syn::Error> {
    let mut fc: Vec<common::FieldWithCardinality> =
        Vec::with_capacity(item_struct.fields.iter().len());

    for field in item_struct.fields.iter() {
        let mut field = field.clone();

        field.attrs = Vec::new();
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
        fc.push(common::FieldWithCardinality { field, cardinality });
    }

    Ok(fc)
}

pub fn struct_arrow_builder(
    message: syn::ItemStruct,
) -> Result<proc_macro::TokenStream, syn::Error> {
    let message_instance_ident: syn::Ident = syn::parse_quote!(record);

    let builder_ident = syn::Ident::new(&format!("{}Builder", message.ident), message.ident.span());
    let builder_vis = message.vis.clone();

    let fc_list = generate_builder_fields(&message)?;

    let append_value_fn = generators::generate_struct_append_value_fn(
        &builder_vis,
        &message_instance_ident,
        &message.ident,
        &fc_list,
    );
    let append_null_fn = generators::generate_append_null_fn(&builder_vis, &fc_list);
    let append_option_fn = generators::generate_append_option_fn(
        &builder_vis,
        &message_instance_ident,
        &message.ident,
    );

    let finish_fn =
        generators::generate_finish_fn(&builder_vis, &fc_list, common::MessageDataType::Struct);
    let finish_cloned_fn = generators::generate_finish_cloned_fn(
        &builder_vis,
        &fc_list,
        common::MessageDataType::Struct,
    );

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
    let builder_fields: Vec<syn::Field> = fc_list.into_iter().map(|f| f.field).collect();

    Ok((quote::quote! {
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
