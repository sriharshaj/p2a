use heck::ToSnakeCase;
use quote::ToTokens;
use std::error;
use syn::spanned::Spanned;

fn oneof_enum(item: &syn::ItemEnum) -> bool {
    //TODO: change later to proper code
    for v in &item.variants {
        if v.fields.is_empty() {
            return false;
        }
    }

    true
}

pub(super) fn generate_arrow_builders(
    proto_items: Vec<syn::Item>,
    namespace: &str,
) -> Result<Vec<syn::Item>, Box<dyn error::Error>> {
    let mut builder_items: Vec<syn::Item> = Vec::new();

    for item in proto_items {
        match item {
            syn::Item::Struct(data) => {
                let builder = Builder::try_from((&data, namespace))?;
                let mut items = builder.generate_all_items();
                builder_items.append(&mut items);
            }
            syn::Item::Enum(data) => {
                if !oneof_enum(&data) {
                    continue;
                }
                let builder = Builder::try_from((&data, namespace))?;
                let mut items = builder.generate_all_items();
                builder_items.append(&mut items);
            }
            syn::Item::Mod(mut data) => {
                let namespace = format!("{}::{}", namespace, &data.ident);
                data.attrs = Vec::new();
                if let Some((brace, items)) = data.content {
                    data.content = Some((brace, generate_arrow_builders(items, &namespace)?));
                }
                builder_items.push(syn::Item::Mod(data));
            }
            _ => {}
        }
    }
    Ok(builder_items)
}

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

fn parse_generic_argument(
    argument: &syn::GenericArgument,
    enum_path: Option<syn::Path>,
) -> Result<syn::Type, syn::Error> {
    if let syn::GenericArgument::Type(ty) = argument
        && let syn::Type::Path(type_path) = ty
    {
        let (t, _, _) = parse_type(enum_path, type_path.to_owned())?;
        Ok(syn::parse_quote!(#t))
    } else {
        Err(syn::Error::new(
            argument.span(),
            "Expected protobuf compatible type",
        ))
    }
}

fn parse_single_arg_path(
    arguments: &syn::PathArguments,
    enum_path: Option<syn::Path>,
) -> Result<syn::Type, syn::Error> {
    if let syn::PathArguments::AngleBracketed(args) = arguments
        && let Some(first_arg) = args.args.first()
    {
        parse_generic_argument(first_arg, enum_path)
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
            parse_generic_argument(&args.args[0], None)?,
            parse_generic_argument(&args.args[1], None)?,
        ))
    } else {
        Err(syn::Error::new(
            arguments.span(),
            "Expected two arguments in angle brackets for map type",
        ))
    }
}

pub fn parse_type(
    enum_path: Option<syn::Path>,
    mut type_path: syn::TypePath,
) -> Result<(syn::Type, FieldRule, ProtoType), syn::Error> {
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
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "f32" {
        Ok((
            syn::parse_quote!(::arrow::array::Float32Builder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "f64" {
        Ok((
            syn::parse_quote!(::arrow::array::Float64Builder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "i32" {
        if let Some(enum_path) = enum_path {
            return Ok((
                syn::parse_quote!(::arrow::array::StringBuilder),
                FieldRule::Required,
                ProtoType::Enum(enum_path),
            ));
        }
        Ok((
            syn::parse_quote!(::arrow::array::Int32Builder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "i64" {
        Ok((
            syn::parse_quote!(::arrow::array::Int64Builder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "u8" {
        Ok((
            syn::parse_quote!(::arrow::array::BinaryBuilder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "u32" {
        Ok((
            syn::parse_quote!(::arrow::array::UInt32Builder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if full_path == "u64" {
        Ok((
            syn::parse_quote!(::arrow::array::UInt64Builder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if is_option(&full_path) {
        Ok((
            parse_single_arg_path(&last_segment.arguments, enum_path)?,
            FieldRule::Optional,
            ProtoType::Scalar,
        ))
    } else if is_string_type(&full_path) {
        Ok((
            syn::parse_quote!(::arrow::array::StringBuilder),
            FieldRule::Required,
            ProtoType::Scalar,
        ))
    } else if is_vec_type(&full_path) {
        let t = parse_single_arg_path(&last_segment.arguments, enum_path)?;
        if let syn::Type::Path(type_path) = &t {
            let full_path = get_full_path(type_path);
            if full_path == "::arrow::array::BinaryBuilder" {
                return Ok((t, FieldRule::Required, ProtoType::Scalar));
            }
        }

        Ok((
            syn::parse_quote!(::arrow::array::ListBuilder<#t>),
            FieldRule::Repeated,
            ProtoType::Scalar,
        ))
    } else if is_box_type(&full_path) {
        Err(syn::Error::new(
            type_path.span(),
            "Box types are not yet supported",
        ))
    } else if is_hash_map_type(&full_path) {
        let (k, v) = parse_two_args_path(&last_segment.arguments)?;
        let t = syn::parse_quote!(::arrow::array::MapBuilder<#k,#v>);
        Ok((t, FieldRule::Required, ProtoType::Map))
    } else {
        let segment = syn::PathSegment {
            ident: syn::Ident::new(
                &(last_segment.ident.to_string() + "Builder"),
                last_segment.span(),
            ),
            arguments: syn::PathArguments::None,
        };

        type_path.path.segments.push(segment);
        Ok((
            syn::parse_quote!(#type_path),
            FieldRule::Required,
            ProtoType::Message,
        ))
    }
}

fn is_enum_proto_type(attrs: Vec<syn::Attribute>, namespace: &str) -> Option<syn::Path> {
    if attrs.is_empty() {
        panic!("expected atleast one attribute");
    }
    let attr = attrs.last().unwrap().to_owned();

    if attr.style != syn::AttrStyle::Outer {
        panic!("expected outer attribute style");
    }

    let meta_list = match attr.meta {
        syn::Meta::List(list) => list,
        _ => panic!("expected meta list"),
    };

    let path = meta_list.path.to_token_stream().to_string();
    if path != "prost" {
        panic!("expected meta to start with prost, got: {}", path);
    }

    let args_list = meta_list.tokens.to_string();
    let (first_arg, _) = args_list
        .split_once(",")
        .expect("expected two args inside meta list");

    let (proto_type, type_path) = first_arg.split_once("=").unwrap_or((first_arg, ""));
    if proto_type.trim() == "enumeration" {
        println!("{}", type_path);
        return Some(
            syn::parse_str(&format!(
                "{}::{}",
                namespace,
                type_path.trim().trim_matches('"')
            ))
            .unwrap(),
        );
    }
    None
}

pub fn generate_fields_from_struct(
    item_struct: &syn::ItemStruct,
    namespace: &str,
) -> Result<Vec<Field>, Box<dyn error::Error>> {
    let mut fc: Vec<Field> = Vec::with_capacity(item_struct.fields.iter().len());

    for field in item_struct.fields.iter() {
        let mut field = field.clone();

        let enum_path = is_enum_proto_type(field.attrs, namespace);
        println!("{:?}", enum_path);
        field.attrs = Vec::new();
        let (ty, field_rule, proto_type) = match field.ty {
            syn::Type::Path(type_path) => parse_type(enum_path, type_path)?,
            _ => {
                return Err("Expected protobuf compatible type".into());
            }
        };
        field.ty = ty;
        fc.push(Field {
            value: field,
            field_rule,
            proto_type,
            enum_variant: None,
        });
    }

    Ok(fc)
}

fn generate_field_from_enum_variant(
    vis: &syn::Visibility,
    variant: &syn::Variant,
) -> Result<Field, syn::Error> {
    if variant.fields.len() != 1 {
        return Err(syn::Error::new(
            variant.ident.span(),
            format!(
                "Enum variants must have exactly one field.
                This macro is designed for prost-generated enums from protobuf oneof fields: {:?}",
                variant
            ),
        ));
    }

    let mut field = variant.fields.iter().next().unwrap().clone();

    field.attrs = Vec::new();
    field.vis = vis.clone();
    field.colon_token = Some(syn::token::Colon::default());
    field.ident = Some(syn::Ident::new(
        &variant.ident.to_string().to_snake_case(),
        variant.ident.span(),
    ));

    let (ty, field_rule, proto_type) = match field.ty {
        syn::Type::Path(type_path) => parse_type(None, type_path)?,
        _ => {
            return Err(syn::Error::new(
                field.ty.span(),
                "Expected protobuf compatible type",
            ));
        }
    };
    field.ty = ty;

    Ok(Field {
        value: field,
        field_rule,
        proto_type,
        enum_variant: Some(variant.ident.clone()),
    })
}

#[derive(PartialEq)]
pub enum FieldRule {
    Optional,
    Repeated,
    Required,
}

#[derive(PartialEq)]
pub enum ProtoType {
    #[allow(dead_code)]
    Enum(syn::Path),
    Message,
    Scalar,
    Map,
}

pub struct Field {
    pub value: syn::Field,
    pub field_rule: FieldRule,
    pub proto_type: ProtoType,
    pub enum_variant: Option<syn::Ident>,
}

impl Field {
    fn generate_append_value_stmt(
        &self,
        message_type: &MessageType,
        message_instance: &syn::Ident,
    ) -> syn::Stmt {
        let field_ident = &self.value.ident.as_ref().unwrap();
        let field_expr: syn::Expr = match message_type {
            MessageType::Enum => syn::parse_quote!(#message_instance),
            MessageType::Struct => syn::parse_quote!(#message_instance.#field_ident),
        };

        match &self.proto_type {
            ProtoType::Map => {
                syn::parse_quote! {
                    if #field_expr.is_empty() {
                        let _ = self.#field_ident.append(false);
                    } else {
                        for (key, val) in #field_expr.into_iter() {
                            self.#field_ident.keys().append_value(key);
                            self.#field_ident.values().append_value(val);
                        }
                        let _ = self.#field_ident.append(true);
                    }
                }
            }
            ProtoType::Enum(enum_path) => {
                syn::parse_quote! {
                  self.#field_ident.append_value(#enum_path::try_from(#field_expr).unwrap().as_str_name());
                }
            }
            _ => match self.field_rule {
                FieldRule::Required => {
                    syn::parse_quote! {
                        self.#field_ident.append_value(#field_expr);
                    }
                }
                FieldRule::Optional => {
                    syn::parse_quote! {
                        self.#field_ident.append_option(#field_expr);
                    }
                }
                FieldRule::Repeated => {
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
}

pub enum MessageType {
    Enum,
    Struct,
}
pub struct Builder {
    message: syn::Path,
    message_instance: syn::Ident,
    ident: syn::Ident,
    message_type: MessageType,
    fields: Vec<Field>,
}

impl TryFrom<(&syn::ItemStruct, &str)> for Builder {
    type Error = Box<dyn error::Error>;
    fn try_from((value, namespace): (&syn::ItemStruct, &str)) -> Result<Self, Self::Error> {
        let fields = generate_fields_from_struct(value, namespace)?;
        // let message: syn::Ident = syn::parse_str(&format!("{}::{}", namespace, value.ident))
        //     .map_err(|e| format!("Failed to parse {}::{}", namespace, value.ident))?;
        Ok(Builder {
            message: syn::parse_str(&format!("{}::{}", namespace, value.ident))?,
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

impl TryFrom<(&syn::ItemEnum, &str)> for Builder {
    type Error = Box<dyn error::Error>;
    fn try_from((value, namespace): (&syn::ItemEnum, &str)) -> Result<Self, Self::Error> {
        let mut fields = Vec::new();
        for v in &value.variants {
            fields.push(generate_field_from_enum_variant(&value.vis, v)?);
        }
        // let message: syn::Ident = syn::parse_str(&format!("{}::{}", namespace, value.ident))
        //     .map_err(|e| format!("Failed to parse {}::{}", namespace, value.ident))?;
        Ok(Builder {
            message: syn::parse_str(&format!("{}::{}", namespace, value.ident))?,
            message_instance: syn::parse_quote!(record),
            ident: syn::Ident::new(
                &format!("{}Builder", value.ident),
                proc_macro2::Span::call_site(),
            ),
            message_type: MessageType::Enum,
            fields,
        })
    }
}

impl Builder {
    fn generate_all_items(&self) -> Vec<syn::Item> {
        let builder_struct = self.generate_struct();
        let append_value_fn = self.generate_append_value_fn();
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
        let builder_fields: Vec<syn::Field> = self.fields.iter().map(|f| f.value.clone()).collect();

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
            .map(|f| f.generate_append_value_stmt(&self.message_type, message_instance))
            .collect();

        syn::parse_quote! {
            pub fn append_value(&mut self, #message_instance: #message) {
                #(#stmts)*

                self._nulls.append(true);
            }
        }
    }

    fn generate_enum_append_value_fn(&self) -> syn::ImplItemFn {
        assert!(matches!(self.message_type, MessageType::Enum));
        let (message_instance, message) = (&self.message_instance, &self.message);

        let arms: Vec<syn::Arm> = self
            .fields
            .iter()
            .map(|matched_field| {
                let variant_ident = matched_field.enum_variant.as_ref().unwrap();

                let append_stmt =
                    matched_field.generate_append_value_stmt(&self.message_type, message_instance);

                let null_stmts: Vec<syn::Stmt> = self
                    .fields
                    .iter()
                    .filter(|f| f.enum_variant != matched_field.enum_variant)
                    .map(|f| {
                        let other_field_ident = f.value.ident.as_ref().unwrap();
                        syn::parse_quote! {
                            self.#other_field_ident.append_null();
                        }
                    })
                    .collect();

                syn::parse_quote! {
                    #message::#variant_ident(#message_instance) => {
                        #append_stmt
                        #(#null_stmts)*
                    }
                }
            })
            .collect();

        syn::parse_quote! {
            pub fn append_value(&mut self, #message_instance: #message) {
                match #message_instance {
                    #(#arms)*
                }

                self._nulls.append_non_null();
            }
        }
    }

    fn generate_append_value_fn(&self) -> syn::ImplItemFn {
        match self.message_type {
            MessageType::Struct => self.generate_struct_append_value_fn(),
            MessageType::Enum => self.generate_enum_append_value_fn(),
        }
    }

    fn generate_append_null_fn(&self) -> syn::ImplItemFn {
        let stmts: Vec<syn::Stmt> = self
            .fields
            .iter()
            .map(|f| {
                let ident = f.value.ident.clone().unwrap();
                match f.proto_type {
                    ProtoType::Map => {
                        syn::parse_quote! {
                            let _ = self.#ident.append(false);
                        }
                    }
                    _ => {
                        syn::parse_quote! {
                          self.#ident.append_null();
                        }
                    }
                }
            })
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
            .map(|field| {
                let ident = field.value.ident.clone().unwrap();
                let ident_str = ident.to_string();
                let ident_str = ident_str.strip_prefix("r#").unwrap_or(&ident_str);

                let is_nullable = match self.message_type {
                    MessageType::Struct => match field.proto_type {
                        ProtoType::Map => true,
                        _ => !(field.field_rule == FieldRule::Required),
                    },
                    MessageType::Enum => true,
                };
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
            })
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
            .map(|f| match f.proto_type {
                ProtoType::Map => {
                    let ident = f.value.ident.clone().unwrap();
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
                    let ident = f.value.ident.clone().unwrap();
                    syn::FieldValue {
                        attrs: Vec::new(),
                        member: syn::Member::Named(ident),
                        colon_token: Some(syn::token::Colon::default()),
                        expr: syn::parse_quote!(Default::default()),
                    }
                }
            })
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
