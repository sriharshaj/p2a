use std::collections::HashMap;

use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::ToTokens;

use crate::{proto, utils::get_absolute_type_path};

fn is_oneof_enum(item: &syn::ItemEnum) -> bool {
    //TODO: change later to proper code
    for v in &item.variants {
        if v.fields.is_empty() {
            return false;
        }
    }

    true
}

pub fn parse_oneof(
    proto_items: &[syn::Item],
    proto_namespace: &str,
    arrow_namespace: &str,
) -> syn::Result<HashMap<String, Vec<proto::Field>>> {
    let mut oneof_enums: HashMap<String, Vec<proto::Field>> = HashMap::new();

    for item in proto_items {
        if let syn::Item::Enum(data) = item {
            if !is_oneof_enum(data) {
                continue;
            }

            let mut fields = Vec::new();
            for v in &data.variants {
                fields.push(parse_to_proto_field(v, proto_namespace, arrow_namespace)?);
            }
            let path = get_absolute_type_path(&data.ident.to_string(), arrow_namespace);
            oneof_enums.insert(path, fields);
        } else if let syn::Item::Mod(data) = item {
            let proto_namespace = format!("{}::{}", proto_namespace, &data.ident);
            let arrow_namespace = format!("{}::{}", arrow_namespace, &data.ident);
            if let Some((_, items)) = &data.content {
                oneof_enums.extend(parse_oneof(items, &proto_namespace, &arrow_namespace)?);
            }
        }
    }

    Ok(oneof_enums)
}

pub(super) fn generate_arrow_builders(
    proto_items: Vec<syn::Item>,
    proto_namespace: &str,
    arrow_namespace: &str,
    one_of_enums: &HashMap<String, Vec<proto::Field>>,
) -> syn::Result<Vec<syn::Item>> {
    let mut builder_items: Vec<syn::Item> = Vec::new();

    for item in proto_items {
        match item {
            syn::Item::Struct(data) => {
                let builder =
                    Builder::try_from((&data, proto_namespace, arrow_namespace, one_of_enums))?;
                let mut items = builder.generate_all_items();
                builder_items.append(&mut items);
            }
            syn::Item::Mod(mut data) => {
                let proto_namespace = format!("{}::{}", proto_namespace, &data.ident);
                let arrow_namespace = format!("{}::{}", arrow_namespace, &data.ident);
                data.attrs = Vec::new();
                if let Some((brace, items)) = data.content {
                    data.content = Some((
                        brace,
                        generate_arrow_builders(
                            items,
                            &proto_namespace,
                            &arrow_namespace,
                            one_of_enums,
                        )?,
                    ));
                }
                builder_items.push(syn::Item::Mod(data));
            }
            _ => {}
        }
    }
    Ok(builder_items)
}

pub fn get_arrow_type(
    proto_type: &proto::Type,
    proto_cardinality: &proto::Cardinality,
) -> syn::Type {
    if let proto::Cardinality::Repeated = proto_cardinality {
        let t = get_arrow_type(proto_type, &proto::Cardinality::Singular);
        return syn::parse_quote!(::arrow::array::ListBuilder<#t>);
    }

    match proto_type {
        proto::Type::Bool => syn::parse_quote!(::arrow::array::BooleanBuilder),
        proto::Type::Float => syn::parse_quote!(::arrow::array::Float32Builder),
        proto::Type::Double => syn::parse_quote!(::arrow::array::Float64Builder),
        proto::Type::Enum(_) => syn::parse_quote!(::arrow::array::StringBuilder),
        proto::Type::Int32 | proto::Type::SInt32 | proto::Type::SFixed32 => {
            syn::parse_quote!(::arrow::array::Int32Builder)
        }
        proto::Type::Int64 | proto::Type::SInt64 | proto::Type::SFixed64 => {
            syn::parse_quote!(::arrow::array::Int64Builder)
        }
        proto::Type::UInt32 | proto::Type::Fixed32 => {
            syn::parse_quote!(::arrow::array::UInt32Builder)
        }
        proto::Type::UInt64 | proto::Type::Fixed64 => {
            syn::parse_quote!(::arrow::array::UInt64Builder)
        }
        proto::Type::String => syn::parse_quote!(::arrow::array::StringBuilder),
        proto::Type::Bytes => syn::parse_quote!(::arrow::array::BinaryBuilder),
        proto::Type::Map(k, v) => {
            let k = get_arrow_type(k, &proto::Cardinality::Singular);
            let v = get_arrow_type(v, &proto::Cardinality::Singular);
            syn::parse_quote!(::arrow::array::MapBuilder<#k,#v>)
        }
        proto::Type::Message(m) => {
            let mut path = m.clone();
            let last_ident = path.segments.last().unwrap().ident.clone();

            let segment = syn::PathSegment {
                ident: syn::Ident::new(&format!("{}Builder", last_ident), last_ident.span()),
                arguments: syn::PathArguments::None,
            };

            path.segments.pop();
            path.segments.push(segment);
            syn::parse_quote!(#path)
        }
        _ => {
            panic!("Unexpected");
        }
    }
}

fn parse_attribute(
    field: &syn::Field,
    proto_namespace: &str,
    arrow_namespace: &str,
    is_oneof_field: bool,
    oneof_fields: &HashMap<String, Vec<proto::Field>>,
) -> syn::Result<proto::Field> {
    let attrs = &field.attrs;
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;

    if attrs.is_empty() {
        panic!("expected atleast one attribute");
    }
    let attr = attrs.last().unwrap();

    if attr.style != syn::AttrStyle::Outer {
        panic!("expected outer attribute style");
    }

    let meta_list = match &attr.meta {
        syn::Meta::List(list) => list,
        _ => panic!("expected meta list"),
    };

    let path = meta_list.path.to_token_stream().to_string();
    if path != "prost" {
        panic!("expected meta to start with prost, got: {}", path);
    }

    syn::parse::Parser::parse2(
        |input: syn::parse::ParseStream| {
            proto::Field::parse_with_namespace(
                input,
                ident,
                ty,
                proto_namespace,
                arrow_namespace,
                is_oneof_field,
                oneof_fields,
            )
        },
        meta_list.tokens.clone(),
    )
}

pub fn parse_to_proto_fields(
    item_struct: &syn::ItemStruct,
    proto_namespace: &str,
    arrow_namespace: &str,
    oneof_fields: &HashMap<String, Vec<proto::Field>>,
) -> syn::Result<Vec<proto::Field>> {
    let mut fields = Vec::new();
    for field in item_struct.fields.iter() {
        let proto_field =
            parse_attribute(field, proto_namespace, arrow_namespace, false, oneof_fields)?;
        fields.push(proto_field);
    }
    Ok(fields)
}

fn parse_to_proto_field(
    variant: &syn::Variant,
    proto_namespace: &str,
    arrow_namespace: &str,
) -> syn::Result<proto::Field> {
    if variant.fields.len() != 1 {
        return Err(syn::Error::new_spanned(
            variant,
            "Enum variants must have exactly one field",
        ));
    }

    let field = syn::Field {
        vis: syn::Visibility::Inherited,
        attrs: variant.attrs.to_owned(),
        mutability: syn::FieldMutability::None,
        ident: syn::parse_str(&variant.ident.to_string().to_snake_case())?,
        colon_token: Some(syn::token::Colon::default()),
        ty: variant.fields.iter().next().unwrap().clone().ty,
    };

    parse_attribute(
        &field,
        proto_namespace,
        arrow_namespace,
        true,
        &HashMap::new(),
    )
}

impl proto::Field {
    fn generate_append_value_stmt(&self, message_instance: &syn::Ident) -> syn::Stmt {
        let field_ident = &self.name;
        let field_expr: syn::Expr = if self.oneof_field {
            syn::parse_quote!(#message_instance)
        } else {
            syn::parse_quote!(#message_instance.#field_ident)
        };

        match &self.r#type {
            proto::Type::Map(_, value_field) => {
                let val: syn::Ident = syn::parse_quote!(val);
                let value_append_stmt: syn::Stmt =
                    if let proto::Type::Enum(enum_path) = value_field.as_ref() {
                        syn::parse_quote! {
                            self.#field_ident.values().append_value(
                                #enum_path::try_from(#val).unwrap_or_default().as_str_name()
                            );
                        }
                    } else {
                        syn::parse_quote! {
                            self.#field_ident.values().append_value(#val);
                        }
                    };

                syn::parse_quote! {
                    if #field_expr.is_empty() {
                        let _ = self.#field_ident.append(false);
                    } else {
                        for (key, #val) in #field_expr.into_iter() {
                            self.#field_ident.keys().append_value(key);
                            #value_append_stmt
                        }
                        let _ = self.#field_ident.append(true);
                    }
                }
            }
            proto::Type::Enum(enum_path) => match self.cardinality {
                proto::Cardinality::Singular => {
                    syn::parse_quote! {
                        self.#field_ident.append_value(
                            #enum_path::try_from(#field_expr).unwrap_or_default().as_str_name()
                        );
                    }
                }
                proto::Cardinality::Optional => {
                    if self.oneof_field {
                        syn::parse_quote! {
                            self.#field_ident.append_value(
                                #enum_path::try_from(#field_expr).unwrap_or_default().as_str_name()
                            );
                        }
                    } else {
                        syn::parse_quote! {
                            self.#field_ident.append_option(#field_expr);
                        }
                    }
                }
                proto::Cardinality::Repeated => {
                    syn::parse_quote! {
                        if #field_expr.is_empty() {
                            self.#field_ident.append_null();
                        } else {
                            self.#field_ident.append_value(#field_expr.into_iter().map(|v| {
                                Some(#enum_path::try_from(v).unwrap_or_default().as_str_name())
                            }));
                        }
                    }
                }
            },
            proto::Type::Oneof(_, _) => self.generate_oneof_append_value_fn(message_instance),
            _ => match self.cardinality {
                proto::Cardinality::Singular => {
                    syn::parse_quote! {
                        self.#field_ident.append_value(#field_expr);
                    }
                }
                proto::Cardinality::Optional => {
                    if self.oneof_field {
                        syn::parse_quote! {
                            self.#field_ident.append_value(#field_expr);
                        }
                    } else {
                        syn::parse_quote! {
                            self.#field_ident.append_option(#field_expr);
                        }
                    }
                }
                proto::Cardinality::Repeated => {
                    syn::parse_quote! {
                        if #field_expr.is_empty() {
                            self.#field_ident.append_null();
                        } else {
                            self.#field_ident.append_value(#field_expr.into_iter().map(Some));
                        }
                    }
                }
            },
        }
    }

    fn generate_oneof_append_value_fn(&self, message_instance: &syn::Ident) -> syn::Stmt {
        let (enum_path, fields) = if let proto::Type::Oneof(enum_path, fields) = &self.r#type {
            (enum_path, fields)
        } else {
            panic!("This can only be called on Oneof types");
        };
        let field_ident = &self.name;

        let arms: Vec<syn::Arm> = fields
            .iter()
            .map(|matched_field| {
                let variant_ident: syn::Ident =
                    syn::parse_str(&matched_field.name.to_string().to_upper_camel_case()).unwrap();

                let append_stmt = matched_field.generate_append_value_stmt(&self.name);
                let null_stmts: Vec<syn::Stmt> = fields
                    .iter()
                    .filter(|f| f.name != matched_field.name)
                    .map(|f| {
                        let other_field_ident = &f.name;
                        syn::parse_quote! {
                            self.#other_field_ident.append_null();
                        }
                    })
                    .collect();

                syn::parse_quote! {
                    #enum_path::#variant_ident(#field_ident) => {
                        #append_stmt
                        #(#null_stmts)*
                    }
                }
            })
            .collect();

        let all_null_stmts: Vec<syn::Stmt> = fields
            .iter()
            .map(|f| {
                let ident = &f.name;
                syn::parse_quote! {
                    self.#ident.append_null();
                }
            })
            .collect();

        syn::parse_quote! {
            if let Some(record) = #message_instance.#field_ident {
                match record {
                    #(#arms)*
                };
            } else {
                #(#all_null_stmts)*
            }
        }
    }

    fn generate_append_null_stmt(&self) -> syn::Stmt {
        let ident = &self.name;
        match &self.r#type {
            proto::Type::Map(_, _) => {
                syn::parse_quote! {
                    let _ = self.#ident.append(false);
                }
            }
            proto::Type::Oneof(_, fields) => {
                let oneof_stmts = fields.iter().map(|f| f.generate_append_null_stmt());
                syn::parse_quote! {
                    {
                        #(#oneof_stmts)*
                    };
                }
            }
            _ => {
                syn::parse_quote! {
                    self.#ident.append_null();
                }
            }
        }
    }

    fn generate_finish_stmt(&self, finish: &syn::Ident) -> syn::Stmt {
        let ident = &self.name;
        let ident_str = ident.to_string();
        let ident_str = ident_str.strip_prefix("r#").unwrap_or(&ident_str);

        let is_nullable = match self.r#type {
            proto::Type::Map(_, _) => true,
            _ => !(self.cardinality == proto::Cardinality::Singular),
        };
        match &self.r#type {
            proto::Type::Oneof(_, fields) => {
                let stmts: Vec<syn::Stmt> = fields
                    .iter()
                    .map(|f| f.generate_finish_stmt(finish))
                    .collect();
                syn::parse_quote! {
                    {
                        #(#stmts)*
                    };
                }
            }
            _ => {
                // Check if this is a Repeated field (List) but NOT a Map
                // Maps are repeated in proto but usually map to MapArray/StructArray, handled differently
                let is_repeated_list = matches!(self.cardinality, proto::Cardinality::Repeated)
                    && !matches!(self.r#type, proto::Type::Map(_, _));

                if is_repeated_list {
                    syn::parse_quote! {
                        {
                            // 1. Finish the builder (returns concrete array type)
                            let raw_array = self.#ident.#finish();
                            // Wrap as generic ArrayRef immediately
                            let mut _array: ::arrow::array::ArrayRef = ::std::sync::Arc::new(raw_array);

                            // 2. Rename inner field "item" -> "element" for Parquet compatibility
                            // We use ArrayData manipulation to be safe against specific List implementations
                            if let ::arrow::datatypes::DataType::List(inner_field) = _array.data_type() {
                                // Create new inner field with name "element"
                                let new_inner_field = ::std::sync::Arc::new(::arrow::datatypes::Field::new(
                                    "element",
                                    inner_field.data_type().clone(),
                                    true // Keep inner elements nullable
                                ));

                                let new_data_type = ::arrow::datatypes::DataType::List(new_inner_field);

                                // Zero-copy rebuild of ArrayData with new DataType
                                let new_data = _array.to_data()
                                    .into_builder()
                                    .data_type(new_data_type)
                                    .build()
                                    .expect("Failed to rebuild array data with renamed field");

                                _array = ::arrow::array::make_array(new_data);
                            }

                            let _field = ::arrow::datatypes::Field::new(
                                #ident_str,
                                ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                                #is_nullable,
                            );

                            arrays.push(_array);
                            fields.push(_field);
                        };
                    }
                } else {
                    syn::parse_quote! {
                        {
                            let _array = ::std::sync::Arc::new(self.#ident.#finish());
                            let _field = ::arrow::datatypes::Field::new(
                                #ident_str,
                                ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                                #is_nullable,
                            );

                            arrays.push(_array);
                            fields.push(_field);
                        };
                    }
                }
            } // _ => {
              //     syn::parse_quote! {
              //         {
              //             let _array = ::std::sync::Arc::new(self.#ident.#finish());
              //             let _field = ::arrow::datatypes::Field::new(
              //                 #ident_str,
              //                 ::arrow::array::Array::data_type(_array.as_ref()).clone(),
              //                 #is_nullable,
              //             );

              //             arrays.push(_array);
              //             fields.push(_field);
              //         };
              //     }
              // }
        }
    }

    fn generate_default_value(&self) -> Vec<syn::FieldValue> {
        match &self.r#type {
            proto::Type::Map(_, _) => {
                let ident = self.name.clone();
                vec![syn::FieldValue {
                    attrs: Vec::new(),
                    member: syn::Member::Named(ident),
                    colon_token: Some(syn::token::Colon::default()),
                    expr: syn::parse_quote!(::arrow::array::MapBuilder::new(
                        None,
                        Default::default(),
                        Default::default()
                    )),
                }]
            }
            proto::Type::Oneof(_, fields) => fields
                .iter()
                .flat_map(|f| f.generate_default_value())
                .collect(),

            _ => {
                let ident = self.name.clone();
                vec![syn::FieldValue {
                    attrs: Vec::new(),
                    member: syn::Member::Named(ident),
                    colon_token: Some(syn::token::Colon::default()),
                    expr: syn::parse_quote!(Default::default()),
                }]
            }
        }
    }
}

pub enum MessageType {
    #[allow(dead_code)]
    Enum,
    Struct,
}
pub struct Builder {
    message: syn::Path,
    message_instance: syn::Ident,
    ident: syn::Ident,
    message_type: MessageType,
    fields: Vec<proto::Field>,
}

impl
    TryFrom<(
        &syn::ItemStruct,
        &str,
        &str,
        &HashMap<String, Vec<proto::Field>>,
    )> for Builder
{
    type Error = syn::Error;
    fn try_from(
        (value, proto_namespace, arrow_namespace, one_of_enums): (
            &syn::ItemStruct,
            &str,
            &str,
            &HashMap<String, Vec<proto::Field>>,
        ),
    ) -> syn::Result<Self> {
        let fields = parse_to_proto_fields(value, proto_namespace, arrow_namespace, one_of_enums)?;
        Ok(Builder {
            message: syn::parse_str(&format!("{}::{}", proto_namespace, value.ident))?,
            message_instance: syn::parse_quote!(record),
            ident: syn::Ident::new(
                &format!("{}Builder", value.ident),
                proc_macro2::Span::call_site(),
            ),
            message_type: MessageType::Struct,
            fields,
        })
    }
}

impl Builder {
    fn generate_all_items(&self) -> Vec<syn::Item> {
        let builder_struct = self.generate_struct();
        let append_value_fn = self.generate_struct_append_value_fn();
        let append_null_fn = self.generate_append_null_fn();
        let append_option_fn = self.generate_append_option_fn();
        let finish_fn = self.generate_finish_fn(false);
        let finish_cloned_fn = self.generate_finish_fn(true);

        let builder = &self.ident;
        let builder_impl: syn::ItemImpl = syn::parse_quote! {
            impl #builder{
                #append_value_fn
                #append_null_fn
                #append_option_fn
                #finish_fn
                #finish_cloned_fn
            }
        };
        let default_trait_impl = self.generate_default_trait_impl();
        let extend_trait_impl = self.generate_extend_trait_impl();
        let array_builder_trait_impl = self.generate_array_builder_trait_impl();

        vec![
            syn::Item::Struct(builder_struct),
            syn::Item::Impl(default_trait_impl),
            syn::Item::Impl(builder_impl),
            syn::Item::Impl(extend_trait_impl),
            syn::Item::Impl(array_builder_trait_impl),
        ]
    }

    fn generate_struct(&self) -> syn::ItemStruct {
        let mut builder_fields: Vec<syn::Field> = Vec::new();

        let non_oneof_fields = self
            .fields
            .iter()
            .filter(|f| !matches!(f.r#type, proto::Type::Oneof(_, _)));

        let oneof_inner_fields = self
            .fields
            .iter()
            .flat_map(|f| match &f.r#type {
                proto::Type::Oneof(_, inner_fields) => Some(inner_fields),
                _ => None,
            })
            .flatten();

        for proto_field in non_oneof_fields.chain(oneof_inner_fields) {
            let ty = get_arrow_type(&proto_field.r#type, &proto_field.cardinality);
            let field = syn::Field {
                attrs: Vec::new(),
                vis: syn::parse_quote!(pub),
                mutability: syn::FieldMutability::None,
                ident: Some(proto_field.name.to_owned()),
                colon_token: Default::default(),
                ty,
            };

            builder_fields.push(field);
        }

        let builder = &self.ident;
        syn::parse_quote! {
            #[derive(Debug)]
            pub struct #builder {
                #(#builder_fields,)*

                _nulls: ::arrow::array::NullBufferBuilder,
            }
        }
    }

    fn generate_struct_append_value_fn(&self) -> syn::ImplItemFn {
        assert!(matches!(self.message_type, MessageType::Struct));
        let (message_instance, message) = (&self.message_instance, &self.message);

        let stmts: Vec<syn::Stmt> = self
            .fields
            .iter()
            .map(|f| f.generate_append_value_stmt(message_instance))
            .collect();

        syn::parse_quote! {
            pub fn append_value(&mut self, #message_instance: #message) {
                #(#stmts)*

                self._nulls.append(true);
            }
        }
    }

    fn generate_append_null_fn(&self) -> syn::ImplItemFn {
        let stmts: Vec<syn::Stmt> = self
            .fields
            .iter()
            .map(|f| f.generate_append_null_stmt())
            .collect();

        syn::parse_quote! {
            pub fn append_null(&mut self) {
                #(#stmts)*

                self._nulls.append_null();
            }
        }
    }

    fn generate_append_option_fn(&self) -> syn::ImplItemFn {
        let (message_instance, message) = (&self.message_instance, &self.message);
        syn::parse_quote! {
            pub fn append_option(&mut self, #message_instance: Option<#message>) {
                match #message_instance {
                    Some(#message_instance) => self.append_value(#message_instance),
                    None => self.append_null(),
                }
            }
        }
    }

    fn get_finish_ident_and_recv(cloned: bool) -> (syn::Ident, syn::Receiver) {
        if cloned {
            return (syn::parse_quote!(finish_cloned), syn::parse_quote!(&self));
        }
        (syn::parse_quote!(finish), syn::parse_quote!(&mut self))
    }

    fn generate_finish_fn(&self, cloned: bool) -> syn::ImplItemFn {
        let (finish, finish_recv) = Self::get_finish_ident_and_recv(cloned);
        let finish_stmts: Vec<syn::Stmt> = self
            .fields
            .iter()
            .map(|f| f.generate_finish_stmt(&finish))
            .collect();

        syn::parse_quote! {
            pub fn #finish(#finish_recv) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();

                #(#finish_stmts)*

                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.#finish(),
                )
            }
        }
    }

    fn generate_default_trait_impl(&self) -> syn::ItemImpl {
        let default_exprs: Vec<syn::FieldValue> = self
            .fields
            .iter()
            .flat_map(|f| f.generate_default_value())
            .collect();

        let builder = &self.ident;
        syn::parse_quote! {
            impl Default for #builder {
                fn default() -> Self {
                    #builder {
                        #(#default_exprs,)*
                        _nulls: ::arrow::array::NullBufferBuilder::new(0),
                    }
                }
            }
        }
    }

    fn generate_extend_trait_impl(&self) -> syn::ItemImpl {
        let message = &self.message;
        let builder = &self.ident;
        syn::parse_quote! {
            impl Extend<Option<#message>> for #builder {
                fn extend<T: IntoIterator<Item = Option<#message>>>(&mut self, iter: T) {
                    iter.into_iter().for_each(|r| self.append_option(r));
                }
            }
        }
    }

    fn generate_array_builder_trait_impl(&self) -> syn::ItemImpl {
        let builder = &self.ident;
        syn::parse_quote! {
            impl ::arrow::array::ArrayBuilder for #builder {
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
}
