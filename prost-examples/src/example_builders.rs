#[derive(Debug)]
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
impl UserBuilder {
    pub fn append_value(&mut self, record: crate::example::User) {
        self.id.append_value(record.id);
        self.name.append_value(record.name);
        self.email.append_value(record.email);
        self.age.append_value(record.age);
        self.is_active.append_value(record.is_active);
        self.r#type.append_value(record.r#type);
        if record.addresses.is_empty() {
            self.addresses.append_null();
        } else {
            self.addresses.append_value(record.addresses.into_iter().map(Some));
        }
        if record.transactions.is_empty() {
            self.transactions.append_null();
        } else {
            self.transactions.append_value(record.transactions.into_iter().map(Some));
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
    pub fn append_option(&mut self, record: Option<crate::example::User>) {
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
                "type",
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
                "type",
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
impl Extend<Option<crate::example::User>> for UserBuilder {
    fn extend<T: IntoIterator<Item = Option<crate::example::User>>>(&mut self, iter: T) {
        iter.into_iter().for_each(|r| self.append_option(r));
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
pub mod user {
    #[derive(Debug)]
    pub struct AddressBuilder {
        pub street: ::arrow::array::StringBuilder,
        pub city: ::arrow::array::StringBuilder,
        pub state: ::arrow::array::StringBuilder,
        pub zip_code: ::arrow::array::StringBuilder,
        pub country: ::arrow::array::StringBuilder,
        _nulls: ::arrow::array::NullBufferBuilder,
    }
    impl Default for AddressBuilder {
        fn default() -> Self {
            AddressBuilder {
                street: Default::default(),
                city: Default::default(),
                state: Default::default(),
                zip_code: Default::default(),
                country: Default::default(),
                _nulls: ::arrow::array::NullBufferBuilder::new(0),
            }
        }
    }
    impl AddressBuilder {
        pub fn append_value(&mut self, record: crate::example::user::Address) {
            self.street.append_value(record.street);
            self.city.append_value(record.city);
            self.state.append_value(record.state);
            self.zip_code.append_value(record.zip_code);
            self.country.append_value(record.country);
            self._nulls.append(true);
        }
        pub fn append_null(&mut self) {
            self.street.append_null();
            self.city.append_null();
            self.state.append_null();
            self.zip_code.append_null();
            self.country.append_null();
            self._nulls.append_null();
        }
        pub fn append_option(&mut self, record: Option<crate::example::user::Address>) {
            match record {
                Some(record) => self.append_value(record),
                None => self.append_null(),
            }
        }
        pub fn finish(&mut self) -> ::arrow::array::StructArray {
            let mut arrays: Vec<::arrow::array::ArrayRef> = Vec::new();
            let mut fields: Vec<::arrow::datatypes::Field> = Vec::new();
            {
                let _array = ::std::sync::Arc::new(self.street.finish());
                let _field = ::arrow::datatypes::Field::new(
                    "street",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.city.finish());
                let _field = ::arrow::datatypes::Field::new(
                    "city",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.state.finish());
                let _field = ::arrow::datatypes::Field::new(
                    "state",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.zip_code.finish());
                let _field = ::arrow::datatypes::Field::new(
                    "zip_code",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.country.finish());
                let _field = ::arrow::datatypes::Field::new(
                    "country",
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
                let _array = ::std::sync::Arc::new(self.street.finish_cloned());
                let _field = ::arrow::datatypes::Field::new(
                    "street",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.city.finish_cloned());
                let _field = ::arrow::datatypes::Field::new(
                    "city",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.state.finish_cloned());
                let _field = ::arrow::datatypes::Field::new(
                    "state",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.zip_code.finish_cloned());
                let _field = ::arrow::datatypes::Field::new(
                    "zip_code",
                    ::arrow::array::Array::data_type(_array.as_ref()).clone(),
                    false,
                );
                arrays.push(_array);
                fields.push(_field);
            };
            {
                let _array = ::std::sync::Arc::new(self.country.finish_cloned());
                let _field = ::arrow::datatypes::Field::new(
                    "country",
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
    impl Extend<Option<crate::example::user::Address>> for AddressBuilder {
        fn extend<T: IntoIterator<Item = Option<crate::example::user::Address>>>(
            &mut self,
            iter: T,
        ) {
            iter.into_iter().for_each(|r| self.append_option(r));
        }
    }
    impl ::arrow::array::ArrayBuilder for AddressBuilder {
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
    #[derive(Debug)]
    pub struct PaymentMethodBuilder {
        pub credit_card: super::CreditCardBuilder,
        pub bank_account: super::BankAccountBuilder,
        _nulls: ::arrow::array::NullBufferBuilder,
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
    impl PaymentMethodBuilder {
        pub fn append_value(&mut self, record: crate::example::user::PaymentMethod) {
            match record {
                crate::example::user::PaymentMethod::CreditCard(record) => {
                    self.credit_card.append_value(record);
                    self.bank_account.append_null();
                }
                crate::example::user::PaymentMethod::BankAccount(record) => {
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
        pub fn append_option(
            &mut self,
            record: Option<crate::example::user::PaymentMethod>,
        ) {
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
    impl Extend<Option<crate::example::user::PaymentMethod>> for PaymentMethodBuilder {
        fn extend<T: IntoIterator<Item = Option<crate::example::user::PaymentMethod>>>(
            &mut self,
            iter: T,
        ) {
            iter.into_iter().for_each(|r| self.append_option(r));
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
#[derive(Debug)]
pub struct PostBuilder {
    pub title: ::arrow::array::StringBuilder,
    pub body: ::arrow::array::BinaryBuilder,
    _nulls: ::arrow::array::NullBufferBuilder,
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
impl PostBuilder {
    pub fn append_value(&mut self, record: crate::example::Post) {
        self.title.append_value(record.title);
        self.body.append_value(record.body);
        self._nulls.append(true);
    }
    pub fn append_null(&mut self) {
        self.title.append_null();
        self.body.append_null();
        self._nulls.append_null();
    }
    pub fn append_option(&mut self, record: Option<crate::example::Post>) {
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
impl Extend<Option<crate::example::Post>> for PostBuilder {
    fn extend<T: IntoIterator<Item = Option<crate::example::Post>>>(&mut self, iter: T) {
        iter.into_iter().for_each(|r| self.append_option(r));
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
#[derive(Debug)]
pub struct TransactionBuilder {
    pub amount: ::arrow::array::Float32Builder,
    pub timestamp: ::arrow::array::UInt64Builder,
    _nulls: ::arrow::array::NullBufferBuilder,
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
impl TransactionBuilder {
    pub fn append_value(&mut self, record: crate::example::Transaction) {
        self.amount.append_value(record.amount);
        self.timestamp.append_value(record.timestamp);
        self._nulls.append(true);
    }
    pub fn append_null(&mut self) {
        self.amount.append_null();
        self.timestamp.append_null();
        self._nulls.append_null();
    }
    pub fn append_option(&mut self, record: Option<crate::example::Transaction>) {
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
impl Extend<Option<crate::example::Transaction>> for TransactionBuilder {
    fn extend<T: IntoIterator<Item = Option<crate::example::Transaction>>>(
        &mut self,
        iter: T,
    ) {
        iter.into_iter().for_each(|r| self.append_option(r));
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
#[derive(Debug)]
pub struct CreditCardBuilder {
    pub card_number: ::arrow::array::StringBuilder,
    pub expiry_month: ::arrow::array::UInt32Builder,
    pub expiry_year: ::arrow::array::UInt32Builder,
    pub cvv: ::arrow::array::StringBuilder,
    _nulls: ::arrow::array::NullBufferBuilder,
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
impl CreditCardBuilder {
    pub fn append_value(&mut self, record: crate::example::CreditCard) {
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
    pub fn append_option(&mut self, record: Option<crate::example::CreditCard>) {
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
impl Extend<Option<crate::example::CreditCard>> for CreditCardBuilder {
    fn extend<T: IntoIterator<Item = Option<crate::example::CreditCard>>>(
        &mut self,
        iter: T,
    ) {
        iter.into_iter().for_each(|r| self.append_option(r));
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
#[derive(Debug)]
pub struct BankAccountBuilder {
    pub account_number: ::arrow::array::StringBuilder,
    pub routing_number: ::arrow::array::StringBuilder,
    pub bank_name: ::arrow::array::StringBuilder,
    _nulls: ::arrow::array::NullBufferBuilder,
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
impl BankAccountBuilder {
    pub fn append_value(&mut self, record: crate::example::BankAccount) {
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
    pub fn append_option(&mut self, record: Option<crate::example::BankAccount>) {
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
impl Extend<Option<crate::example::BankAccount>> for BankAccountBuilder {
    fn extend<T: IntoIterator<Item = Option<crate::example::BankAccount>>>(
        &mut self,
        iter: T,
    ) {
        iter.into_iter().for_each(|r| self.append_option(r));
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
