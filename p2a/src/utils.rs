pub fn get_absolute_type_path(type_path: &str, namespace: &str) -> String {
    //TODO: refactor the logic
    if type_path.starts_with("::prost::") || type_path.split("::").count() == 0 {
        return type_path.to_string();
    }
    let mut n_iter = namespace.rsplit("::");
    let mut tp_iter = type_path.trim().trim_matches('"').rsplit("::");
    let mut resolved_path = String::from(tp_iter.next().unwrap());

    for s in tp_iter {
        if s == "super" && n_iter.next().is_some() {
            continue;
        }
        resolved_path = format!("{}::{}", s, resolved_path);
    }
    for s in n_iter {
        resolved_path = format!("{}::{}", s, resolved_path);
    }

    resolved_path
}

pub fn get_full_path(type_path: &syn::TypePath) -> String {
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

pub fn extract_message_type_path(ty: &syn::Type, namespace: &str) -> syn::Result<syn::Path> {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
    {
        let type_path = match &segment.arguments {
            syn::PathArguments::None => type_path,
            syn::PathArguments::AngleBracketed(args) => {
                if let Some(syn::GenericArgument::Type(syn::Type::Path(inner_type_path))) =
                    args.args.first()
                {
                    inner_type_path
                } else {
                    return Err(syn::Error::new_spanned(
                        ty,
                        "Expected T or Option<T> or Vec<T>",
                    ));
                }
            }
            _ => {
                return Err(syn::Error::new_spanned(ty, "Unexpected path arguments"));
            }
        };
        return syn::parse_str(&get_absolute_type_path(
            &get_full_path(type_path),
            namespace,
        ));
    }
    Err(syn::Error::new_spanned(
        ty,
        "Expected T or Option<T> or Vec<T>",
    ))
}

pub fn extract_map_type_paths(
    ty: &syn::Type,
    namespace: &str,
) -> syn::Result<(syn::Path, syn::Path)> {
    if let syn::Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && let syn::PathArguments::AngleBracketed(args) = &segment.arguments
    {
        if args.args.len() != 2 {
            return Err(syn::Error::new_spanned(
                ty,
                "Map type must have exactly 2 type arguments",
            ));
        }

        let key_path = if let Some(syn::GenericArgument::Type(syn::Type::Path(key_type_path))) =
            args.args.first()
        {
            let absolute_path = get_absolute_type_path(&get_full_path(key_type_path), namespace);
            dbg!(&absolute_path);
            syn::parse_str(&absolute_path)?
        } else {
            return Err(syn::Error::new_spanned(ty, "Invalid key type"));
        };

        let value_path = if let Some(syn::GenericArgument::Type(syn::Type::Path(value_type_path))) =
            args.args.last()
        {
            syn::parse_str(&get_absolute_type_path(
                &get_full_path(value_type_path),
                namespace,
            ))?
        } else {
            return Err(syn::Error::new_spanned(ty, "Invalid value type"));
        };

        return Ok((key_path, value_path));
    }
    Err(syn::Error::new_spanned(ty, "Expected HashMap<K, V>"))
}
