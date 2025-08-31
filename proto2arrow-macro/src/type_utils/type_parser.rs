use syn::spanned::Spanned;

use crate::type_utils::FieldCardinality;

fn get_full_path(type_path: &syn::TypePath) -> String {
    let head = if type_path.path.leading_colon.is_some() {
        "::".to_string()
    } else {
        "".to_string()
    };

    let tail = type_path
        .path
        .segments
        .iter()
        .map(|f| f.ident.to_string())
        .collect::<Vec<String>>()
        .join("::");

    format!("{}{}", head, tail)
}

fn is_string_type(full_ident: &str) -> bool {
    full_ident == "String" || full_ident == "::prost::alloc::string::String"
}

fn is_vec_type(full_ident: &str) -> bool {
    full_ident == "Vec" || full_ident == "::prost::alloc::vec::Vec"
}

fn is_box_type(full_ident: &str) -> bool {
    full_ident == "Box" || full_ident == "::prost::alloc::boxed::Box"
}

fn is_hash_map_type(full_ident: &str) -> bool {
    full_ident == "HashMap" || full_ident == "::std::collections::HashMap"
}

fn is_option(full_ident: &str) -> bool {
    full_ident == "Option" || full_ident == "::core::option::Option"
}

fn parse_generic_argument(argument: &syn::GenericArgument) -> Result<syn::Type, syn::Error> {
    if let syn::GenericArgument::Type(ty) = argument
        && let syn::Type::Path(type_path) = ty
    {
        let (t, _) = parse_type(type_path.to_owned())?;
        Ok(syn::parse_quote!(#t))
    } else {
        Err(syn::Error::new(
            argument.span(),
            "Expected protobuf compatible type",
        ))
    }
}

fn parse_single_arg_path(arguments: &syn::PathArguments) -> Result<syn::Type, syn::Error> {
    if let syn::PathArguments::AngleBracketed(args) = arguments
        && let Some(first_arg) = args.args.first()
    {
        parse_generic_argument(first_arg)
    } else {
        Err(syn::Error::new(
            arguments.span(),
            "Expected single argument for container type",
        ))
    }
}

fn parse_two_args_path(
    arguments: &syn::PathArguments,
) -> Result<(syn::Type, syn::Type), syn::Error> {
    if let syn::PathArguments::AngleBracketed(args) = arguments
        && args.args.len() == 2
    {
        Ok((
            parse_generic_argument(&args.args[0])?,
            parse_generic_argument(&args.args[1])?,
        ))
    } else {
        Err(syn::Error::new(
            arguments.span(),
            "Expected two arguments in angle brackets for map type",
        ))
    }
}

pub fn parse_type(
    mut type_path: syn::TypePath,
) -> Result<(syn::Type, FieldCardinality), syn::Error> {
    let full_path = get_full_path(&type_path);
    let last_segment = match type_path.path.segments.pop() {
        Some(syn::punctuated::Pair::End(segment)) => segment,
        Some(_) => {
            return Err(syn::Error::new(
                type_path.path.span(),
                "Expected type path to end with identifier",
            ));
        }
        None => {
            return Err(syn::Error::new(
                type_path.path.span(),
                "Expected non-empty type, found empty type",
            ));
        }
    };

    if full_path == "bool" {
        Ok((
            syn::parse_quote!(::arrow::array::BooleanBuilder),
            FieldCardinality::Required,
        ))
    } else if full_path == "f32" {
        Ok((
            syn::parse_quote!(::arrow::array::Float32Builder),
            FieldCardinality::Required,
        ))
    } else if full_path == "f64" {
        Ok((
            syn::parse_quote!(::arrow::array::Float64Builder),
            FieldCardinality::Required,
        ))
    } else if full_path == "i32" {
        Ok((
            syn::parse_quote!(::arrow::array::Int32Builder),
            FieldCardinality::Required,
        ))
    } else if full_path == "i64" {
        Ok((
            syn::parse_quote!(::arrow::array::Int64Builder),
            FieldCardinality::Required,
        ))
    } else if full_path == "u8" {
        Ok((
            syn::parse_quote!(::arrow::array::UInt8Builder),
            FieldCardinality::Required,
        ))
    } else if full_path == "u32" {
        Ok((
            syn::parse_quote!(::arrow::array::UInt32Builder),
            FieldCardinality::Required,
        ))
    } else if full_path == "u64" {
        Ok((
            syn::parse_quote!(::arrow::array::UInt64Builder),
            FieldCardinality::Required,
        ))
    } else if is_option(&full_path) {
        Ok((
            parse_single_arg_path(&last_segment.arguments)?,
            FieldCardinality::Optional,
        ))
    } else if is_string_type(&full_path) {
        Ok((
            syn::parse_quote!(::arrow::array::StringBuilder),
            FieldCardinality::Required,
        ))
    } else if is_vec_type(&full_path) {
        let t = parse_single_arg_path(&last_segment.arguments)?;
        Ok((
            syn::parse_quote!(::arrow::array::ListBuilder<#t>),
            FieldCardinality::Repeated,
        ))
    } else if is_box_type(&full_path) {
        Err(syn::Error::new(
            type_path.span(),
            "Box types are not yet supported",
        ))
    } else if is_hash_map_type(&full_path) {
        let (k, v) = parse_two_args_path(&last_segment.arguments)?;
        let t = syn::parse_quote!(::arrow::array::MapBuilder<#k,#v>);
        Ok((t, FieldCardinality::Map))
    } else {
        let segment = syn::PathSegment {
            ident: syn::Ident::new(
                &(last_segment.ident.to_string() + "Builder"),
                last_segment.span(),
            ),
            arguments: syn::PathArguments::None,
        };

        type_path.path.segments.push(segment);
        Ok((syn::parse_quote!(#type_path), FieldCardinality::Required))
    }
}
