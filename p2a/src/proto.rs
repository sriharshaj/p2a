#[derive(Default, Debug, PartialEq)]
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
    Message,
    Enum(syn::Path),
    Map(Box<Type>, Box<Type>),
    Oneof(syn::Path),
}

#[derive(Default, Debug, PartialEq)]
pub enum Cardinality {
    #[default]
    Singular,
    Optional,
    Repeated,
}

#[derive(Default, Debug)]
pub struct Field {
    pub r#type: Type,
    pub cardinality: Cardinality,
}

fn parse_type_from_str(s: &str, namespace: &str) -> syn::Result<Type> {
    let s = s.trim();

    // Check for enumeration(TypeName) format
    if let Some(enum_type) = s
        .strip_prefix("enumeration(")
        .and_then(|s| s.strip_suffix(")"))
    {
        let resolved_path = resolve_type_path(enum_type, namespace);
        return Ok(Type::Enum(syn::parse_str(&resolved_path)?));
    }

    // Check for scalar types
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
        "message" => Ok(Type::Message),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Unknown type: {}", s),
        )),
    }
}

fn resolve_type_path(type_path: &str, namespace: &str) -> String {
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

impl Field {
    pub fn parse_with_namespace(
        input: syn::parse::ParseStream,
        namespace: &str,
    ) -> syn::Result<Self> {
        let mut attr = Field::default();

        loop {
            let ident = input.parse::<syn::Ident>()?;

            match ident.to_string().as_str() {
                "enumeration" => {
                    input.parse::<syn::Token![=]>()?;
                    let path: syn::LitStr = input.parse()?;
                    let resolved_path = resolve_type_path(&path.value(), namespace);
                    attr.r#type = Type::Enum(syn::parse_str(&resolved_path)?);
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

                    let key_type = parse_type_from_str(parts[0], namespace)?;
                    let value_type = parse_type_from_str(parts[1], namespace)?;

                    attr.r#type = Type::Map(Box::new(key_type), Box::new(value_type));
                }
                "oneof" => {
                    input.parse::<syn::Token![=]>()?;
                    let path = input.parse::<syn::LitStr>()?;
                    let resolved_path = resolve_type_path(&path.value(), namespace);
                    attr.r#type = Type::Oneof(syn::parse_str(&resolved_path)?);
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
                    attr.r#type = Type::Message;
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
