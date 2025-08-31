mod generated {
    pub mod example {
        pub struct UserBuilder {
            pub id: ::arrow::array::UInt32Builder,
            pub name: ::arrow::array::StringBuilder,
            pub email: ::arrow::array::StringBuilder,
            pub age: ::arrow::array::UInt32Builder,
            pub is_active: ::arrow::array::BooleanBuilder,
            pub r#type: ::arrow::array::Int32Builder,
            pub addresses: ::arrow::array::ListBuilder<user::AddressBuilder>,
            pub transactions: ::arrow::array::ListBuilder<TransactionBuilder>,
            pub posts: ::arrow::array::MapBuilder<::arrow::array::Int32Builder, PostBuilder>,
            pub payment_method: user::PaymentMethodBuilder,
            _nulls: ::arrow::array::NullBufferBuilder,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for UserBuilder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "id",
                    "name",
                    "email",
                    "age",
                    "is_active",
                    "type",
                    "addresses",
                    "transactions",
                    "posts",
                    "payment_method",
                    "_nulls",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.id,
                    &self.name,
                    &self.email,
                    &self.age,
                    &self.is_active,
                    &self.r#type,
                    &self.addresses,
                    &self.transactions,
                    &self.posts,
                    &self.payment_method,
                    &&self._nulls,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(f, "UserBuilder", names, values)
            }
        }
        impl Default for UserBuilder {
            fn default() -> Self {
                UserBuilder {
                    id: Default::default(),
                    name: Default::default(),
                    email: Default::default(),
                    age: Default::default(),
                    is_active: Default::default(),
                    r#type: Default::default(),
                    addresses: Default::default(),
                    transactions: Default::default(),
                    posts: ::arrow::array::MapBuilder::new(
                        None,
                        Default::default(),
                        Default::default(),
                    ),
                    payment_method: Default::default(),
                    _nulls: ::arrow::array::NullBufferBuilder::new(0),
                }
            }
        }
        impl Extend<Option<User>> for UserBuilder {
            fn extend<T: IntoIterator<Item = Option<User>>>(&mut self, iter: T) {
                iter.into_iter().for_each(|r| self.append_option(r));
            }
        }
        impl UserBuilder {
            pub fn append_value(&mut self, record: User) {
                self.id.append_value(record.id);
                self.name.append_value(record.name);
                self.email.append_value(record.email);
                self.age.append_value(record.age);
                self.is_active.append_value(record.is_active);
                self.r#type.append_value(record.r#type);
                if record.addresses.is_empty() {
                    self.addresses.append_null();
                } else {
                    self.addresses
                        .append_value(record.addresses.into_iter().map(Some));
                }
                if record.transactions.is_empty() {
                    self.transactions.append_null();
                } else {
                    self.transactions
                        .append_value(record.transactions.into_iter().map(Some));
                }
                if record.posts.is_empty() {
                    let _ = self.posts.append(false);
                } else {
                    for (key, val) in record.posts.into_iter() {
                        self.posts.keys().append_value(key);
                        self.posts.values().append_value(val);
                    }
                    let _ = self.posts.append(true);
                }
                self.payment_method.append_option(record.payment_method);
                self._nulls.append(true);
            }
            pub fn append_null(&mut self) {
                self.id.append_null();
                self.name.append_null();
                self.email.append_null();
                self.age.append_null();
                self.is_active.append_null();
                self.r#type.append_null();
                self.addresses.append_null();
                self.transactions.append_null();
                let _ = self.posts.append(false);
                self.payment_method.append_null();
                self._nulls.append_null();
            }
            pub fn append_option(&mut self, record: Option<User>) {
                match record {
                    Some(record) => self.append_value(record),
                    None => self.append_null(),
                }
            }
            pub fn finish(&mut self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.id.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "id",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.name.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "name",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.email.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "email",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.age.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "age",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.is_active.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "is_active",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.r#type.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "r#type",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.addresses.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "addresses",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.transactions.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "transactions",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.posts.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "posts",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.payment_method.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "payment_method",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish(),
                )
            }
            pub fn finish_cloned(&self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.id.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "id",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.name.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "name",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.email.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "email",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.age.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "age",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.is_active.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "is_active",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.r#type.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "r#type",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.addresses.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "addresses",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.transactions.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "transactions",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.posts.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "posts",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.payment_method.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "payment_method",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish_cloned(),
                )
            }
        }
        impl ::arrow::array::ArrayBuilder for UserBuilder {
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
        /// Nested message and enum types in `User`.
        pub mod user {
            pub struct AddressBuilder {
                pub street: ::arrow::array::StringBuilder,
                pub city: ::arrow::array::StringBuilder,
                pub state: ::arrow::array::StringBuilder,
                pub zip_code: ::arrow::array::StringBuilder,
                pub country: ::arrow::array::StringBuilder,
                _nulls: ::arrow::array::NullBufferBuilder,
            }
            pub struct PaymentMethodBuilder {
                pub credit_card: super::CreditCardBuilder,
                pub bank_account: super::BankAccountBuilder,
                _nulls: ::arrow::array::NullBufferBuilder,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PaymentMethodBuilder {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "PaymentMethodBuilder",
                        "credit_card",
                        &self.credit_card,
                        "bank_account",
                        &self.bank_account,
                        "_nulls",
                        &&self._nulls,
                    )
                }
            }
            impl Default for PaymentMethodBuilder {
                fn default() -> Self {
                    PaymentMethodBuilder {
                        credit_card: Default::default(),
                        bank_account: Default::default(),
                        _nulls: ::arrow::array::NullBufferBuilder::new(0),
                    }
                }
            }
            impl Extend<Option<PaymentMethod>> for PaymentMethodBuilder {
                fn extend<T: IntoIterator<Item = Option<PaymentMethod>>>(&mut self, iter: T) {
                    iter.into_iter().for_each(|r| self.append_option(r));
                }
            }
            impl PaymentMethodBuilder {
                pub fn append_value(&mut self, record: PaymentMethod) {
                    match record {
                        PaymentMethod::CreditCard(record) => {
                            self.credit_card.append_value(record);
                            self.bank_account.append_null();
                        }
                        PaymentMethod::BankAccount(record) => {
                            self.bank_account.append_value(record);
                            self.credit_card.append_null();
                        }
                    }
                    self._nulls.append_non_null();
                }
                pub fn append_null(&mut self) {
                    self.credit_card.append_null();
                    self.bank_account.append_null();
                    self._nulls.append_null();
                }
                pub fn append_option(&mut self, record: Option<PaymentMethod>) {
                    match record {
                        Some(record) => self.append_value(record),
                        None => self.append_null(),
                    }
                }
                pub fn finish(&mut self) -> ::arrow::array::StructArray {
                    let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                    let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                    {
                        let _array = ::std::sync::Arc::new(self.credit_card.finish());
                        let _field = ::arrow::datatypes::Field::new(
                            "credit_card",
                            ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                            true,
                        );
                        arrays.push(_array);
                        fields.push(_field);
                    };
                    {
                        let _array = ::std::sync::Arc::new(self.bank_account.finish());
                        let _field = ::arrow::datatypes::Field::new(
                            "bank_account",
                            ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                            true,
                        );
                        arrays.push(_array);
                        fields.push(_field);
                    };
                    ::arrow::array::StructArray::new(
                        ::arrow::datatypes::Fields::from(fields),
                        arrays,
                        self._nulls.finish(),
                    )
                }
                pub fn finish_cloned(&self) -> ::arrow::array::StructArray {
                    let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                    let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                    {
                        let _array = ::std::sync::Arc::new(self.credit_card.finish_cloned());
                        let _field = ::arrow::datatypes::Field::new(
                            "credit_card",
                            ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                            true,
                        );
                        arrays.push(_array);
                        fields.push(_field);
                    };
                    {
                        let _array = ::std::sync::Arc::new(self.bank_account.finish_cloned());
                        let _field = ::arrow::datatypes::Field::new(
                            "bank_account",
                            ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                            true,
                        );
                        arrays.push(_array);
                        fields.push(_field);
                    };
                    ::arrow::array::StructArray::new(
                        ::arrow::datatypes::Fields::from(fields),
                        arrays,
                        self._nulls.finish_cloned(),
                    )
                }
            }
            impl ::arrow::array::ArrayBuilder for PaymentMethodBuilder {
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
        pub struct PostBuilder {
            pub title: ::arrow::array::StringBuilder,
            pub body: ::arrow::array::ListBuilder<::arrow::array::UInt8Builder>,
            _nulls: ::arrow::array::NullBufferBuilder,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PostBuilder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "PostBuilder",
                    "title",
                    &self.title,
                    "body",
                    &self.body,
                    "_nulls",
                    &&self._nulls,
                )
            }
        }
        impl Default for PostBuilder {
            fn default() -> Self {
                PostBuilder {
                    title: Default::default(),
                    body: Default::default(),
                    _nulls: ::arrow::array::NullBufferBuilder::new(0),
                }
            }
        }
        impl Extend<Option<Post>> for PostBuilder {
            fn extend<T: IntoIterator<Item = Option<Post>>>(&mut self, iter: T) {
                iter.into_iter().for_each(|r| self.append_option(r));
            }
        }
        impl PostBuilder {
            pub fn append_value(&mut self, record: Post) {
                self.title.append_value(record.title);
                if record.body.is_empty() {
                    self.body.append_null();
                } else {
                    self.body.append_value(record.body.into_iter().map(Some));
                }
                self._nulls.append(true);
            }
            pub fn append_null(&mut self) {
                self.title.append_null();
                self.body.append_null();
                self._nulls.append_null();
            }
            pub fn append_option(&mut self, record: Option<Post>) {
                match record {
                    Some(record) => self.append_value(record),
                    None => self.append_null(),
                }
            }
            pub fn finish(&mut self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.title.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "title",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.body.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "body",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish(),
                )
            }
            pub fn finish_cloned(&self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.title.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "title",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.body.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "body",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        true,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish_cloned(),
                )
            }
        }
        impl ::arrow::array::ArrayBuilder for PostBuilder {
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
        pub struct TransactionBuilder {
            pub amount: ::arrow::array::Float32Builder,
            pub timestamp: ::arrow::array::UInt64Builder,
            _nulls: ::arrow::array::NullBufferBuilder,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TransactionBuilder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "TransactionBuilder",
                    "amount",
                    &self.amount,
                    "timestamp",
                    &self.timestamp,
                    "_nulls",
                    &&self._nulls,
                )
            }
        }
        impl Default for TransactionBuilder {
            fn default() -> Self {
                TransactionBuilder {
                    amount: Default::default(),
                    timestamp: Default::default(),
                    _nulls: ::arrow::array::NullBufferBuilder::new(0),
                }
            }
        }
        impl Extend<Option<Transaction>> for TransactionBuilder {
            fn extend<T: IntoIterator<Item = Option<Transaction>>>(&mut self, iter: T) {
                iter.into_iter().for_each(|r| self.append_option(r));
            }
        }
        impl TransactionBuilder {
            pub fn append_value(&mut self, record: Transaction) {
                self.amount.append_value(record.amount);
                self.timestamp.append_value(record.timestamp);
                self._nulls.append(true);
            }
            pub fn append_null(&mut self) {
                self.amount.append_null();
                self.timestamp.append_null();
                self._nulls.append_null();
            }
            pub fn append_option(&mut self, record: Option<Transaction>) {
                match record {
                    Some(record) => self.append_value(record),
                    None => self.append_null(),
                }
            }
            pub fn finish(&mut self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.amount.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "amount",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.timestamp.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "timestamp",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish(),
                )
            }
            pub fn finish_cloned(&self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.amount.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "amount",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.timestamp.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "timestamp",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish_cloned(),
                )
            }
        }
        impl ::arrow::array::ArrayBuilder for TransactionBuilder {
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
        pub struct CreditCardBuilder {
            pub card_number: ::arrow::array::StringBuilder,
            pub expiry_month: ::arrow::array::UInt32Builder,
            pub expiry_year: ::arrow::array::UInt32Builder,
            pub cvv: ::arrow::array::StringBuilder,
            _nulls: ::arrow::array::NullBufferBuilder,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for CreditCardBuilder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "CreditCardBuilder",
                    "card_number",
                    &self.card_number,
                    "expiry_month",
                    &self.expiry_month,
                    "expiry_year",
                    &self.expiry_year,
                    "cvv",
                    &self.cvv,
                    "_nulls",
                    &&self._nulls,
                )
            }
        }
        impl Default for CreditCardBuilder {
            fn default() -> Self {
                CreditCardBuilder {
                    card_number: Default::default(),
                    expiry_month: Default::default(),
                    expiry_year: Default::default(),
                    cvv: Default::default(),
                    _nulls: ::arrow::array::NullBufferBuilder::new(0),
                }
            }
        }
        impl Extend<Option<CreditCard>> for CreditCardBuilder {
            fn extend<T: IntoIterator<Item = Option<CreditCard>>>(&mut self, iter: T) {
                iter.into_iter().for_each(|r| self.append_option(r));
            }
        }
        impl CreditCardBuilder {
            pub fn append_value(&mut self, record: CreditCard) {
                self.card_number.append_value(record.card_number);
                self.expiry_month.append_value(record.expiry_month);
                self.expiry_year.append_value(record.expiry_year);
                self.cvv.append_value(record.cvv);
                self._nulls.append(true);
            }
            pub fn append_null(&mut self) {
                self.card_number.append_null();
                self.expiry_month.append_null();
                self.expiry_year.append_null();
                self.cvv.append_null();
                self._nulls.append_null();
            }
            pub fn append_option(&mut self, record: Option<CreditCard>) {
                match record {
                    Some(record) => self.append_value(record),
                    None => self.append_null(),
                }
            }
            pub fn finish(&mut self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.card_number.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "card_number",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.expiry_month.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "expiry_month",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.expiry_year.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "expiry_year",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.cvv.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "cvv",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish(),
                )
            }
            pub fn finish_cloned(&self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.card_number.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "card_number",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.expiry_month.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "expiry_month",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.expiry_year.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "expiry_year",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.cvv.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "cvv",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish_cloned(),
                )
            }
        }
        impl ::arrow::array::ArrayBuilder for CreditCardBuilder {
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
        pub struct BankAccountBuilder {
            pub account_number: ::arrow::array::StringBuilder,
            pub routing_number: ::arrow::array::StringBuilder,
            pub bank_name: ::arrow::array::StringBuilder,
            _nulls: ::arrow::array::NullBufferBuilder,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for BankAccountBuilder {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "BankAccountBuilder",
                    "account_number",
                    &self.account_number,
                    "routing_number",
                    &self.routing_number,
                    "bank_name",
                    &self.bank_name,
                    "_nulls",
                    &&self._nulls,
                )
            }
        }
        impl Default for BankAccountBuilder {
            fn default() -> Self {
                BankAccountBuilder {
                    account_number: Default::default(),
                    routing_number: Default::default(),
                    bank_name: Default::default(),
                    _nulls: ::arrow::array::NullBufferBuilder::new(0),
                }
            }
        }
        impl Extend<Option<BankAccount>> for BankAccountBuilder {
            fn extend<T: IntoIterator<Item = Option<BankAccount>>>(&mut self, iter: T) {
                iter.into_iter().for_each(|r| self.append_option(r));
            }
        }
        impl BankAccountBuilder {
            pub fn append_value(&mut self, record: BankAccount) {
                self.account_number.append_value(record.account_number);
                self.routing_number.append_value(record.routing_number);
                self.bank_name.append_value(record.bank_name);
                self._nulls.append(true);
            }
            pub fn append_null(&mut self) {
                self.account_number.append_null();
                self.routing_number.append_null();
                self.bank_name.append_null();
                self._nulls.append_null();
            }
            pub fn append_option(&mut self, record: Option<BankAccount>) {
                match record {
                    Some(record) => self.append_value(record),
                    None => self.append_null(),
                }
            }
            pub fn finish(&mut self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.account_number.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "account_number",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.routing_number.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "routing_number",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.bank_name.finish());
                    let _field = ::arrow::datatypes::Field::new(
                        "bank_name",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish(),
                )
            }
            pub fn finish_cloned(&self) -> ::arrow::array::StructArray {
                let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
                let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
                {
                    let _array = ::std::sync::Arc::new(self.account_number.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "account_number",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.routing_number.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "routing_number",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                {
                    let _array = ::std::sync::Arc::new(self.bank_name.finish_cloned());
                    let _field = ::arrow::datatypes::Field::new(
                        "bank_name",
                        ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                        false,
                    );
                    arrays.push(_array);
                    fields.push(_field);
                };
                ::arrow::array::StructArray::new(
                    ::arrow::datatypes::Fields::from(fields),
                    arrays,
                    self._nulls.finish_cloned(),
                )
            }
        }
        impl ::arrow::array::ArrayBuilder for BankAccountBuilder {
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
