pub struct FieldWithCardinality {
    pub field: syn::Field,
    pub cardinality: FieldCardinality,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldCardinality {
    Required,
    Optional,
    Repeated,
    Map,
}

pub struct EnumVariant<'a> {
    pub ident: &'a syn::Ident,
    pub fc: FieldWithCardinality,
}

pub enum MessageDataType {
    Enum,
    Struct,
}
