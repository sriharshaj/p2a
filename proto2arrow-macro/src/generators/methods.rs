use crate::type_utils as common;

fn generate_append_stmt(
    field_ident: &syn::Ident,
    field_expr: &syn::Expr,
    cardinality: &common::FieldCardinality,
) -> syn::Stmt {
    match cardinality {
        common::FieldCardinality::Required => {
            syn::parse_quote! {
                self.#field_ident.append_value(#field_expr);
            }
        }
        common::FieldCardinality::Optional => {
            syn::parse_quote! {
                self.#field_ident.append_option(#field_expr);
            }
        }
        common::FieldCardinality::Repeated => {
            syn::parse_quote! {
                if #field_expr.is_empty() {
                    self.#field_ident.append_null();
                } else {
                    self.#field_ident.append_value(#field_expr.into_iter().map(Some));
                }
            }
        }
        common::FieldCardinality::Map => {
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
    }
}

pub fn generate_enum_append_value_fn(
    vis: &syn::Visibility,
    enum_instance_ident: &syn::Ident,
    enum_ident: &syn::Ident,
    variants: &Vec<common::EnumVariant>,
) -> syn::ImplItemFn {
    let arms: Vec<syn::Arm> = variants
        .iter()
        .map(|current_variant| {
            let variant_ident = current_variant.ident;
            let field_ident = current_variant.fc.field.ident.as_ref().unwrap();
            let field_expr: syn::Expr = syn::parse_quote!(#enum_instance_ident);

            let append_stmt =
                generate_append_stmt(field_ident, &field_expr, &current_variant.fc.cardinality);

            let null_stmts: Vec<syn::Stmt> = variants
                .iter()
                .filter(|v| v.ident != current_variant.ident)
                .map(|v| {
                    let other_field_ident = v.fc.field.ident.as_ref().unwrap();
                    syn::parse_quote! {
                        self.#other_field_ident.append_null();
                    }
                })
                .collect();

            syn::parse_quote! {
                #enum_ident::#variant_ident(#enum_instance_ident) => {
                    #append_stmt
                    #(#null_stmts)*
                }
            }
        })
        .collect();

    syn::parse_quote! {
        #vis fn append_value(&mut self, #enum_instance_ident: #enum_ident) {
            match #enum_instance_ident {
                #(#arms)*
            }

            self._nulls.append_non_null();
        }
    }
}

pub fn generate_struct_append_value_fn(
    vis: &syn::Visibility,
    message_instance_ident: &syn::Ident,
    message_ident: &syn::Ident,
    fc: &[common::FieldWithCardinality],
) -> syn::ImplItemFn {
    let stmts: Vec<syn::Stmt> = fc
        .iter()
        .map(|f| {
            let field_ident = &f.field.ident.as_ref().unwrap();
            let field_expr: syn::Expr = syn::parse_quote!(#message_instance_ident.#field_ident);

            generate_append_stmt(field_ident, &field_expr, &f.cardinality)
        })
        .collect();

    syn::parse_quote! {
        #vis fn append_value(&mut self, #message_instance_ident: #message_ident) {
            #(#stmts)*

            self._nulls.append(true);
        }
    }
}

pub fn generate_append_null_fn(
    vis: &syn::Visibility,
    fc: &[common::FieldWithCardinality],
) -> syn::ImplItemFn {
    let stmts: Vec<syn::Stmt> = fc
        .iter()
        .map(|f| {
            let ident = f.field.ident.clone().unwrap();
            match f.cardinality {
                common::FieldCardinality::Map => {
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
        #vis fn append_null(&mut self) {
            #(#stmts)*

            self._nulls.append_null();
        }
    }
}

pub fn generate_append_option_fn(
    vis: &syn::Visibility,
    message_instance_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> syn::ImplItemFn {
    syn::parse_quote! {
        #vis fn append_option(&mut self, #message_instance_ident: Option<#message_ident>) {
            match #message_instance_ident {
                Some(#message_instance_ident) => self.append_value(#message_instance_ident),
                None => self.append_null(),
            }
        }
    }
}

pub fn generate_finish_fn(
    vis: &syn::Visibility,
    fc_list: &[common::FieldWithCardinality],
    data_type: common::MessageDataType,
) -> syn::ImplItemFn {
    let finish_stmts: Vec<syn::Stmt> = fc_list
        .iter()
        .map(|fc| {
            let ident = fc.field.ident.clone().unwrap();
            let ident_str = ident.to_string();
            let ident_str = ident_str.strip_prefix("r#").unwrap_or(&ident_str);

            let is_nullable = match data_type {
                common::MessageDataType::Struct => {
                    fc.cardinality != common::FieldCardinality::Required
                }
                common::MessageDataType::Enum => true,
            };
            syn::parse_quote! {
                {
                    let _array = ::std::sync::Arc::new(self.#ident.finish());
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
        #vis fn finish(&mut self) -> ::arrow::array::StructArray {
            let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
            let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();

            #(#finish_stmts)*

            ::arrow::array::StructArray::new(
                ::arrow::datatypes::Fields::from(fields),
                arrays,
                self._nulls.finish(),
            )
        }
    }
}

pub fn generate_finish_cloned_fn(
    vis: &syn::Visibility,
    fc_list: &[common::FieldWithCardinality],
    data_type: common::MessageDataType,
) -> syn::ImplItemFn {
    let finish_cloned_stmts: Vec<syn::Stmt> = fc_list
        .iter()
        .map(|fc| {
            let ident = fc.field.ident.clone().unwrap();
            let ident_str = ident.to_string();
            let ident_str = ident_str.strip_prefix("r#").unwrap_or(&ident_str);

            let is_nullable = match data_type {
                common::MessageDataType::Struct => {
                    fc.cardinality != common::FieldCardinality::Required
                }
                common::MessageDataType::Enum => true,
            };
            syn::parse_quote! {
                {
                    let _array = ::std::sync::Arc::new(self.#ident.finish_cloned());
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
        #vis fn finish_cloned(&self) -> ::arrow::array::StructArray {
            let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
            let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();

            #(#finish_cloned_stmts)*

            ::arrow::array::StructArray::new(
                ::arrow::datatypes::Fields::from(fields),
                arrays,
                self._nulls.finish_cloned(),
            )
        }
    }
}
