use std::collections::HashMap;

use crate::utils::{extract_map_type_paths, extract_message_type_path, get_absolute_type_path};

#[derive(Default, Debug, Clone)]
pub enum Type {
    #[default]
    Unknown,
    Int32,
    Int64,
    UInt32,
    UInt64,
    SInt32,
    SInt64,
    Fixed32,
    Fixed64,
    SFixed32,
    SFixed64,
    Float,
    Double,
    Bool,
    String,
    Bytes,
    Message(syn::Path),
    Enum(syn::Path),
    Map(Box<Type>, Box<Type>),
    Oneof(syn::Path, Vec<Field>),
}

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Cardinality {
    #[default]
    Singular,
    Optional,
    Repeated,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: syn::Ident,
    pub r#type: Type,
    pub cardinality: Cardinality,
    // If field is inside oneof arm
    pub oneof_field: bool,
}

fn parse_type_from_str(s: &str, proto_namespace: &str, type_path: syn::Path) -> syn::Result<Type> {
    let s = s.trim();
    dbg!(s);

    // Check for enumeration(TypeName) format
    if let Some(enum_type) = s
        .strip_prefix("enumeration(")
        .and_then(|s| s.strip_suffix(")"))
    {
        let resolved_path = get_absolute_type_path(enum_type, proto_namespace);
        return Ok(Type::Enum(syn::parse_str(&resolved_path)?));
    }

    match s {
        "int32" => Ok(Type::Int32),
        "int64" => Ok(Type::Int64),
        "uint32" => Ok(Type::UInt32),
        "uint64" => Ok(Type::UInt64),
        "sint32" => Ok(Type::SInt32),
        "sint64" => Ok(Type::SInt64),
        "fixed32" => Ok(Type::Fixed32),
        "fixed64" => Ok(Type::Fixed64),
        "sfixed32" => Ok(Type::SFixed32),
        "sfixed64" => Ok(Type::SFixed64),
        "float" => Ok(Type::Float),
        "double" => Ok(Type::Double),
        "bool" => Ok(Type::Bool),
        "string" => Ok(Type::String),
        "bytes" => Ok(Type::Bytes),
        "message" => Ok(Type::Message(type_path)),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unknown type: {}", s),
        )),
    }
}

impl Field {
    pub fn parse_with_namespace(
        input: syn::parse::ParseStream,
        ident: &syn::Ident,
        ty: &syn::Type,
        proto_namespace: &str,
        arrow_namespace: &str,
        is_oneof_field: bool,
        oneof_fields: &HashMap<String, Vec<Field>>,
    ) -> syn::Result<Self> {
        let mut attr = Field {
            name: ident.to_owned(),
            r#type: Default::default(),
            cardinality: Default::default(),
            oneof_field: is_oneof_field,
        };

        loop {
            let ident = input.parse::<syn::Ident>()?;

            match ident.to_string().as_str() {
                "enumeration" => {
                    input.parse::<syn::Token![=]>()?;
                    let path: syn::LitStr = input.parse()?;
                    let path = get_absolute_type_path(&path.value(), proto_namespace);
                    attr.r#type = Type::Enum(syn::parse_str(&path)?);
                }
                "map" => {
                    input.parse::<syn::Token![=]>()?;
                    let map_types: syn::LitStr = input.parse()?;

                    let types_str = map_types.value();
                    let parts: Vec<&str> = types_str.split(',').collect();

                    if parts.len() != 2 {
                        return Err(syn::Error::new(
                            map_types.span(),
                            "Map must have key and value",
                        ));
                    }

                    let (key_path, val_path) = extract_map_type_paths(ty, arrow_namespace)?;
                    let key_type = parse_type_from_str(parts[0], proto_namespace, key_path)?;
                    let value_type = parse_type_from_str(parts[1], proto_namespace, val_path)?;

                    attr.r#type = Type::Map(Box::new(key_type), Box::new(value_type));
                }
                "oneof" => {
                    input.parse::<syn::Token![=]>()?;
                    let path = input.parse::<syn::LitStr>()?;
                    let path = get_absolute_type_path(&path.value(), arrow_namespace);
                    attr.r#type = Type::Oneof(
                        extract_message_type_path(ty, proto_namespace)?,
                        oneof_fields.get(&path).unwrap().clone(),
                    );
                    attr.cardinality = Cardinality::Optional;
                }
                "bytes" => {
                    attr.r#type = Type::Bytes;
                    input.parse::<syn::Token![=]>()?;
                    input.parse::<syn::LitStr>()?;
                }
                "repeated" => {
                    attr.cardinality = Cardinality::Repeated;
                }
                "optional" => {
                    attr.cardinality = Cardinality::Optional;
                }
                "int32" => {
                    attr.r#type = Type::Int32;
                }
                "int64" => {
                    attr.r#type = Type::Int64;
                }
                "uint32" => {
                    attr.r#type = Type::UInt32;
                }
                "uint64" => {
                    attr.r#type = Type::UInt64;
                }
                "sint32" => {
                    attr.r#type = Type::SInt32;
                }
                "sint64" => {
                    attr.r#type = Type::SInt64;
                }
                "fixed32" => {
                    attr.r#type = Type::Fixed32;
                }
                "fixed64" => {
                    attr.r#type = Type::Fixed64;
                }
                "sfixed32" => {
                    attr.r#type = Type::SFixed32;
                }
                "sfixed64" => {
                    attr.r#type = Type::SFixed64;
                }
                "float" => {
                    attr.r#type = Type::Float;
                }
                "double" => {
                    attr.r#type = Type::Double;
                }
                "bool" => {
                    attr.r#type = Type::Bool;
                }
                "string" => {
                    attr.r#type = Type::String;
                }
                "message" => {
                    attr.cardinality = Cardinality::Optional;
                    attr.r#type = Type::Message(extract_message_type_path(ty, arrow_namespace)?);
                }
                "tags" | "tag" => {
                    input.parse::<syn::Token![=]>()?;
                    input.parse::<syn::Lit>()?;
                    break;
                }
                _ => return Err(syn::Error::new(ident.span(), "Unknown keyword")),
            }
            if !input.peek(syn::Token![,]) {
                break;
            }
            input.parse::<syn::Token![,]>()?;
        }

        Ok(attr)
    }
}
